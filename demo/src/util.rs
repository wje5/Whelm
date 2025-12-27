use std::io;

pub fn readline() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s
}