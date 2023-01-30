mod dedup_logic;

use std::{env, error::Error};

fn help() -> Result<(), Box<dyn Error>> {
    println!("Usage:");
    println!("dedup <path>");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return help();
    }
    simple_logger::init_with_env()?;
    // Run main logic
    dedup_logic::run(args[1].trim().to_string())?;
    Ok(())
}
