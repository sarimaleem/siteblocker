use std::fs::{OpenOptions, self};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/hosts")?;

    writeln!(file, "### BLOCKLIST START")?;
    input.lines().for_each(|s| {
        println!("blocking      {}", s);
        let _ = writeln!(file, "127.0.0.1       {}", s);
    });
    writeln!(file, "### BLOCKLIST END")?;

    Ok(())
}
