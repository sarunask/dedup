mod dedup_logic;

use std::{env, error::Error, str::FromStr};

fn help() -> Result<(), Box<dyn Error>> {
    println!("Usage:");
    println!("dedup <path>");
    Ok(())
}

fn set_log_level() -> Result<(), Box<dyn Error>> {
    let mut level_to_init = log::Level::Info;
    if let Ok(env_level) = env::var("LOG_LEVEL") {
        level_to_init = log::Level::from_str(env_level.as_str())?;
    }
    simple_logger::init_with_level(level_to_init)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return help();
    }
    set_log_level()?;
    // Run main logic
    dedup_logic::run(args[1].trim().to_string())?;
    Ok(())
}
