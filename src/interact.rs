use std::io::{self, BufRead, Write};
use crate::command;
use crate::kv_store::KVStore;

pub fn prompt() {
    let mut kv_store = KVStore::new();
    let stdin = io::stdin();

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let line_result  = stdin.lock().lines().next();
        if line_result.is_none() {
            panic!("Exiting program");
        }
        
        match line_result.expect("no line found") {
            Ok(line) => process_command(&mut kv_store, line),
            Err(err) => println!("Error reading input, {}", err)
        }
    }
}


fn handle_set(kv_store: &mut KVStore, args: Vec<&str>) {
    if args[1..].len() != 2 {
        println!("Invalid number of args provided for get");
        return;
    }

    kv_store.set(args[1], args[2]);
}

fn handle_get_or_unset(kv_store: &mut KVStore, args: Vec<&str>) {
    if args[1..].len() != 1 {
        println!("Invalid number of args provided for unset");
        return;
    }
    
    kv_store.execute(args[0], args[1]);
}

fn process_command(kv_store: &mut KVStore, line: String) {
    let input_command = line.trim();
    let args = input_command.split_whitespace().collect::<Vec<&str>>();
    match args[0] {
        command::GET => handle_get_or_unset(kv_store, args),
        command::UNSET => handle_get_or_unset(kv_store, args),
        command::SET => handle_set(kv_store, args),
        command::EXIT => {
            println!("Exiting");
            panic!("Exiting");
        }
        _ => {
            println!("Invalid command");
        }
    }
}
