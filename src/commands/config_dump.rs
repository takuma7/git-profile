use anyhow::{Result};
use std::{fs::File, io::{BufRead, BufReader}};

fn open(filename: &str) -> Result<Box<dyn BufRead>> { 
    Ok(Box::new(BufReader::new(File::open(filename)?)))
}

pub fn execute(config_path: &str) -> Result<()> {
    let buf = open(config_path)?;

    buf.lines().for_each(|line| {
        println!("{}", line.unwrap());
    });

    Ok(())
}
