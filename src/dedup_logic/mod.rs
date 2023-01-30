use ahash::AHashMap;
use checksum::crc::Crc as crc;
use log::{debug};
use std::{
    collections::hash_map::Entry::{Occupied, Vacant},
    error::Error,
};
use walkdir::WalkDir;

pub struct Duplicates {
    map: AHashMap<u64, Vec<String>>,
}

impl Duplicates {
    pub fn new() -> Duplicates {
        Duplicates {
            map: AHashMap::new(),
        }
    }

    pub fn add(&mut self, filename: String) -> Result<u64, &str> {
        let mut crc = crc::new(&filename);
        // get checksum or return error
        let checksum = crc.checksum().unwrap();
        // add checksum into hashmap
        match self.map.entry(checksum.crc64) {
            Occupied(mut entry) => {
                entry.get_mut().push(filename);
            }
            Vacant(_) => {
                self.map.entry(checksum.crc64).or_insert(vec![filename]);
            }
        }
        Ok(checksum.crc64)
    }

    pub fn print(&self) {
        for (key, value) in &self.map {
            if value.len() > 1 {
                println!("duplicates for crc {key}:");
                for val in value.iter() {
                    println!("{val}");
                }
            }
        }
    }
}

pub fn run(dir: String) -> Result<(), Box<dyn Error>> {
    debug!("Using input dir: {}", dir);
    let mut dups = Duplicates::new();
    for file in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            debug!("{}", file.path().display());
            let f_path = format!("{}", file.path().display());
            dups.add(f_path)?;
        }
    }
    dups.print();
    Ok(())
}
