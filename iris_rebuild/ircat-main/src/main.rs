use std::{net::{IpAddr, TcpStream}, io::{BufRead, Write}, sync::mpsc::channel, thread, process};
use bufstream::BufStream;

fn main() {
    let mut args = std::env::args().skip(1);
    let address: IpAddr = {
        let arg = args.next().unwrap_or_else(|| "127.0.0.1".to_string());
        arg.parse()
            .expect(&format!("invalid address: {arg}"))
    }; 

    let port: u16 = {
        let arg = args.next().unwrap_or_else(|| "6991".to_string());
        arg.parse()
            .expect(&format!("invalid port: {arg}"))
    };

    let stream_read = TcpStream::connect((address, port))
        .expect(&format!("failed to connect to {address}:{port}"));
    let mut stream_write = stream_read.try_clone().expect("failed to clone connection");

    let mut stream_read = BufStream::new(stream_read);

    let (send, recv) = channel::<String>();

    // network thread read
    {
        thread::spawn(move || {
            loop {
                let mut line = String::new();
                match stream_read.read_line(&mut line) {
                    Ok(0) => {
                        // EOF
                        eprintln!("server stopped responding");
                        process::exit(1);
                    }
                    Err(e) => {
                        eprintln!("reader failed: {e:?}");
                        return;
                    }
                    Ok(_) => {
                        let line = line.trim();

                        println!("{line}");
                    }
                }
            }
        });
    }

    // network thread write
    {
        thread::spawn(move || {
            loop {
                match recv.recv() {
                    Ok(message) => {
                        if let Err(_) = stream_write.write_all(message.as_bytes())
                            .and_then(|_| stream_write.flush()) {
                            
                            eprintln!("writer failed");
                        }
                    }
                    Err(_) => {
                        // hang up
                        return;
                    }
                }
            }
        });
    }

    // input thread
    {
        let stdin = std::io::stdin();
        loop {
            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(0) => {
                    // EOF
                    process::exit(0);
                }
                Err(_) => {
                    eprintln!("failed to read further input");
                    return;
                }
                Ok(_) => {
                    line = line.trim().to_string();
                    line.extend("\r\n".chars());
                    send.send(line).unwrap();
                }
            }
        }
    }
}
