use std::fs::{self, OpenOptions};
use std::io::{prelude::*, Error};

const START_DELIMETER: &str = "### BLOCKLIST START";
const END_DELIMETER: &str = "### BLOCKLIST END";

fn main() {
    let result = write_blocklist();
    match result {
        Ok(_) => println!("blocklist written"),
        Err(_) => println!("Failed"),
    };
    delete_blocklist();
}

fn write_blocklist() -> std::io::Result<()> {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/hosts")?;

    writeln!(file, "{}", START_DELIMETER)?;
    input.lines().for_each(|s| {
        println!("blocking      {}", s);
        let _ = writeln!(file, "127.0.0.1       {}", s);
    });
    writeln!(file, "{}", END_DELIMETER)?;
    Ok(())
}

fn delete_blocklist() -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).open("/etc/hosts")?;

    let input = fs::read_to_string("/etc/hosts").unwrap();

    let mut lines = Vec::new();
    input.lines().for_each(|s| {
        lines.push(s);
    });

    // Basic checks
    let start_index = lines.iter().position(|s| s == &START_DELIMETER).ok_or("start not found");
    let end_index = lines.iter().position(|s| s == &END_DELIMETER).ok_or("start not found");

    if start_index > end_index {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "hello"));
    }

    lines.drain(start_index..=end_index);

    Ok(())
}
