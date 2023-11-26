use std::env;
use std::fs;
use std::fs::create_dir;
use std::io;
use std::io::prelude::Write;
use std::path::Path;
use std::process::Command;
use std::process::exit;
use std::thread::park_timeout;
use std::time::Duration;
use configparser::ini::Ini;

use crate::app;

fn clear() {
    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        Command::new("bash").args(["-c","clear"]).spawn().unwrap();
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C","cls"]).spawn().unwrap();
    }
    park_timeout(Duration::new(1, 0));
}

fn cmd() {
    clear();
    if !Path::new("config.ini").exists() {
        let mut config = Ini::new();
        let mut username: String = String::new();
        let mut password: String = String::new();
        let mut stdout = io::stdout();
        println!("\x1b[0;35m--- Command Prompt ---\x1b[0m");
        write!(stdout,"\n\x1b[0;32mNew Username \x1b[1;32m-> ").unwrap();
        stdout.flush().unwrap();
        io::stdin().read_line(&mut username).unwrap();
        while username.trim().contains(" ") || username.trim() == "" {
            println!("\x1b[0;31mUsername cant be empty !\x1b[0m\n");
            write!(stdout,"\x1b[0;32mNew Username \x1b[1;32m-> ").unwrap();
            stdout.flush().unwrap();
            io::stdin().read_line(&mut username).unwrap();
            print!("\x1b[0m");
        }
        write!(stdout,"\n\x1b[0;36mNew Password \x1b[1;36m-> ").unwrap();
        stdout.flush().unwrap();
        io::stdin().read_line(&mut password).unwrap();
        while password.trim().contains(" ") || password.trim() == "" {
            println!("\x1b[0;31mPassword cant be empty !\x1b[0m\n");
            write!(stdout,"\x1b[0;36mNew Password \x1b[1;36m-> ").unwrap();
            stdout.flush().unwrap();
            io::stdin().read_line(&mut password).unwrap();
            print!("\x1b[0m");
        }

        println!("Please wait");
        password = app::hash_password(password.trim().to_owned());
        username = app::encode(username.trim().to_owned());

        config.set("app", "username", Some(username.trim().to_owned()));
        config.set("app", "password", Some(password.trim().to_owned()));
        config.write("config.ini").unwrap();
    } else {
        let mut config = Ini::new();
        config.read(fs::read_to_string("config.ini").unwrap()).unwrap();
        let mut username: String = String::new();
        let mut password: String = String::new();
        let mut stdout = io::stdout();
        write!(stdout,"\n\x1b[0;32mUsername \x1b[1;32m-> ").unwrap();
        stdout.flush().unwrap();
        io::stdin().read_line(&mut username).unwrap();
        while !app::decode(config.get("app", "username").unwrap() ,username.trim().to_owned()) {
            println!("\x1b[0;31mUsername doesnt match !\x1b[0m\n");
            write!(stdout,"\n\x1b[0;32mUsername \x1b[1;32m-> ").unwrap();
            stdout.flush().unwrap();
            username = "".to_owned();
            io::stdin().read_line(&mut username).unwrap();
        }
        write!(stdout,"\n\x1b[0;36mPassword \x1b[1;36m-> ").unwrap();
        stdout.flush().unwrap();
        io::stdin().read_line(&mut password).unwrap();
        while !app::verify_password(password.trim().to_owned(), config.get("app","password").unwrap()) {
            println!("\x1b[0;31mPassword doesnt match !\x1b[0m");
            write!(stdout,"\n\x1b[0;36mPassword \x1b[1;36m-> ").unwrap();
            stdout.flush().unwrap();
            password = "".to_owned();
            io::stdin().read_line(&mut password).unwrap();
        }
    }

    clear();

    println!("\n\x1b[0;36m--- Command Prompt ---\x1b[0;32m");
    println!("     Made in Rust     ");
    let mut config = Ini::new();
    config.read(fs::read_to_string("config.ini").unwrap()).unwrap();

    loop {
        let mut stdout = io::stdout();
        let mut input: String = String::new();
        
        write!(stdout,"\n\x1b[0;32m{}\x1b[1;35m:/\x1b[0;36m{}\x1b[1;33m: ",env::current_dir().unwrap().display(), app::decode_string(config.get("app", "username").unwrap())).unwrap();
        stdout.flush().unwrap();
        io::stdin().read_line(&mut input).expect("failed to readline");
        print!("\x1b[0m");

        let mut args = input.trim().split(" ");
        let args_count = input.trim().split(" ").count();

        if args_count == 2 {
            let first_arg = args.next().unwrap();
            let second_arg = args.next().unwrap();

            if (first_arg == "cd") && !second_arg.is_empty() {
                let pt = Path::new(second_arg);
                if pt.exists() {
                    env::set_current_dir(pt).unwrap();
                } else {
                    println!("\x1b[0;31mFailed changing directory to `{}` !\x1b[0m", second_arg);
                }
                continue;
            } else if (first_arg == "mkdir") && !second_arg.is_empty() {
                let pt = Path::new(second_arg);
                if !pt.exists(){
                    create_dir(pt).unwrap();
                } else {
                    println!("\x1b[0;31mA directory named `{}` already exists !\x1b[0m", second_arg);
                }
            }
        }

        match &*input.trim() {
            "help" | "?" => {
                println!("\n\x1b[1;36mhelp:\x1b[0;32m");
                println!("    help              Shows this menu");
                println!("    cd                Change current working directory");
                println!("    clear             Clear the terminal");
                println!("    mkdir             Create a directory");
                println!("    exit              Exits with Code 0");
                print!("\x1b[0m");
            }
            "cd" => {
                println!("\n\x1b[1;36mcd:\x1b[0;32m");
                println!("    Use with one path argument to change working directory");
                println!("    Example: cd ..");
                print!("\x1b[0m");
            }
            "mkdir" => {
                println!("\n\x1b[1;36mmkdir:\x1b[0;32m");
                println!("    Use with one path argument to create a directory");
                println!("    Example: mkdir name");
                print!("\x1b[0m");
            }
            "clear" => clear(),
            "exit" => { print!("\x1b[0m"); println!("\nExit:\n  -> Code 0\n"); exit(0); }
            _ => (),
        }
    }
}

pub fn show() {
    loop {
        clear();
        println!("\x1b[1;36m--- Menu ---\x1b[0;32m");
        println!("\n1. Command Prompt");
        println!("2. SSH");
        println!("3. exit");
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            println!("4. run makefile")
        }
        println!();
        let mut input: String = String::new();
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[0;32mInput ->\x1b[0;36m ").unwrap();
        stdout.flush().unwrap();
        io::stdin().read_line(&mut input).expect("failed to readline");
        print!("\x1b[0m");

        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            match &*input.trim() {
                "1" | "cmd" | "CMD" => cmd(),
                "2" | "ssh" | "SSH" => app::sh(),
                "3" | "exit" => exit(0),
                "4" | "make" => {
                    Command::new("bash").args(["-c","make"]).spawn().unwrap();
                }
                _ => (),
            }
        } else {
            match &*input.trim() {
                "1" | "cmd" | "CMD" => cmd(),
                "2" | "ssh" | "SSH" => app::sh(),
                "3" | "exit" => exit(0),
                _ => (),
            }
        }
    }
}