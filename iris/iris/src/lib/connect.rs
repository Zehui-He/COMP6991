use std::{
    io::{Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream}, error::Error, fmt::{Display, Debug},
};

pub struct ConnectionManager {
    listener: TcpListener,
}

impl ConnectionManager {
    pub fn launch(address: impl Into<IpAddr>, port: u16) -> Self {
        let address = address.into();
        let listener = TcpListener::bind((address, port))
            .unwrap_or_else(|_| panic!("failed to bind to {address}:{port}"));

        Self { listener }
    }

    pub fn accept_new_connection(&mut self) -> (ConnectionRead, ConnectionWrite) {
        loop {
            match self.listener.accept() {
                Ok((socket, addr)) => {
                    let (socket_read, socket_write) = (
                        match socket.try_clone() {
                            Ok(socket) => socket,
                            Err(err) => {
                                eprintln!("[WARN] Failed to clone socket: {err}");
                                continue;
                            }
                        },
                        socket
                    );

                    return (
                        ConnectionRead::from_socket(socket_read, addr),
                        ConnectionWrite::from_socket(socket_write, addr)
                    );
                }
                Err(err) => {
                    eprintln!("[WARN] failed to connect to client: {err}");
                }
            }
        }
    }
}

pub struct ConnectionRead {
    socket: TcpStream,
    socket_addr: SocketAddr,
    buffer: Box<[u8; 512]>,
    buflen: usize,
}

pub struct ConnectionWrite {
    socket: TcpStream,
    socket_addr: SocketAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionError {
    ConnectionLost,
    ConnectionClosed,
    MessageTooLong,
    MessageInvalidUtf8,
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Error for ConnectionError {}

impl ConnectionRead {
    fn from_socket(socket: TcpStream, socket_addr: SocketAddr) -> Self {
        Self {
            socket,
            socket_addr,
            buffer: Box::from([0; 512]),
            buflen: 0,
        }
    }

    fn buffer_crlf(&self) -> Option<usize> {
        self.buffer[..self.buflen]
            .windows(2)
            .enumerate()
            .find(|(_, bytes)| bytes[0] == b'\r' && bytes[1] == b'\n')
            .map(|(index, _)| index)
    }

    pub fn read_message(&mut self) -> Result<String, ConnectionError> {
        use std::io::ErrorKind;

        if self.buffer_crlf().is_none() {
            let n_bytes = loop {
                break match self.socket.read(&mut self.buffer[self.buflen..]) {
                    Ok(0) => return Err(ConnectionError::ConnectionClosed),
                    Ok(n_bytes) => n_bytes,
                    Err(err) => {
                        match err.kind() {
                            // Retry `read` if interrupted...
                            ErrorKind::Interrupted => continue,
                            _ => return Err(ConnectionError::ConnectionLost),
                        }
                    }
                };
            };
    
            self.buflen += n_bytes;
        }

        let end = self.buffer_crlf()
            .ok_or_else(|| {
                // Clear out their data...
                self.buflen = 0;
                ConnectionError::MessageTooLong
            })?;

        let bytes = Vec::from(&self.buffer[0..end]);

        // end + '\r' + '\n'
        let after_crlf = end + 2;

        self.buffer.copy_within(after_crlf..self.buflen, 0);
        self.buflen = self.buflen - after_crlf;

        let message = String::from_utf8(bytes).map_err(|_| ConnectionError::MessageInvalidUtf8)?;

        Ok(message)
    }

    pub fn id(&self) -> String {
        self.socket_addr.to_string()
    }
}

impl ConnectionWrite {
    fn from_socket(socket: TcpStream, socket_addr: SocketAddr) -> Self {
        Self {
            socket,
            socket_addr,
        }
    }

    pub fn write_message(&mut self, message: &str) -> Result<(), ConnectionError> {
        self.socket
            .write_all(message.as_bytes())
            .map_err(|_| ConnectionError::ConnectionClosed)?;
        let _ = self.socket.flush();

        Ok(())
    }

    pub fn id(&self) -> String {
        self.socket_addr.to_string()
    }
}
