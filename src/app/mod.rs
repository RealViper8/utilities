pub mod menu;
use std::{io::{self, Write}, process::Command, thread::park_timeout, time::Duration, os::unix::process::CommandExt};
use base64::{self, engine::general_purpose, Engine};

pub fn hash_password(password: String) -> String {
    bcrypt::hash(password, 14).unwrap()
}

pub fn verify_password(password: String, hash: String) -> bool {
    bcrypt::verify(password, &hash).unwrap()
}

pub fn encode(value: String) -> String {
    general_purpose::STANDARD.encode(value)
}

pub fn decode(encoded_string: String, value: String) -> bool {
    if String::from_utf8(general_purpose::STANDARD.decode(encoded_string).unwrap()).unwrap() == value {
        true
    } else {
        false
    }
}

pub fn sh() {
    let mut host: String = String::new();
    let mut stdout = io::stdout();
    write!(stdout,"\x1b[0;36mHost ->\x1b[1;36m ").unwrap();
    stdout.flush().unwrap();
    io::stdin().read_line(&mut host).expect("failed to readline");
    while host.trim() == "" || host.trim().contains(" ") {
        host = "".to_owned();
        println!("\x1b[1;31mError \x1b[0;31m: Host cant be empty !\x1b[0m\n");
        write!(stdout,"\x1b[0;36mHost ->\x1b[1;36m ").unwrap();
        stdout.flush().unwrap();
        io::stdin().read_line(&mut host).expect("failed to readline");
    }
    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        Command::new("ssh").arg(host.trim()).exec();
    } else if cfg!(target_os = "windows") {
        Command::new("ssh").arg(host.trim()).exec();
    } else {
        println!("\n\x1b[0;31mYour os isnt supported !\x1b[0m");
        park_timeout(Duration::new(2, 0));
    }
}

pub fn decode_string(encoded_string: String) -> String {
    String::from_utf8(general_purpose::STANDARD.decode(encoded_string).unwrap()).unwrap()
}