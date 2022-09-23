fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.contains(pattern) {
            println!("{}", line);
        } else if line.is_empty() {
            break;
        }
    }
}
