use regex::bytes::Regex;
use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
    path::Path,
    process::{Command, Output},
};

fn netcat_command() -> io::Result<Output> {
    Command::new("nc")
        .arg("neo.challenges.cybersecuritychallenge.be")
        .arg("1338")
        .output()
}

pub fn write_bytes_to_file(bytes: Vec<u8>, path: &Path) -> io::Result<()> {
    let bytes = netcat_command()?.stdout;
    let mut file = OpenOptions::new().create(true).write(true).open(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

pub fn remove_resets() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(&Path::new("raw_bytes.bin"))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let re = Regex::new(r"\x1B\[\d+;\d+H").unwrap();
    let output = re.replace_all(&bytes, b"");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&Path::new("no-returns.bin"))?;
    file.write_all(&output).unwrap();
    println!("{}", String::from_utf8(output.to_vec()).unwrap());
    Ok(())
}

pub fn filter() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(&Path::new("no-returns.bin"))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let re: Regex = Regex::new(r"(\x1B\[(\x39\x32|\x30|\x39\x31)m)+(?<letter>.)").unwrap();
    let output = re.captures_iter(&bytes);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&Path::new("new.bin"))?;
    //file.write_all(&output).unwrap();
    for i in output {
        file.write_all(&i["letter"]).unwrap();
        println!("{}", String::from_utf8(i[0].to_vec()).unwrap());
    }
    Ok(())
}

pub fn run() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(&Path::new("raw_bytes.bin"))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let re: Regex = Regex::new(r"\x1B\[\d+;\d+H(\x1B\[(\x39\x32)m)+(?<letter>.)").unwrap();
    let output = re.captures_iter(&bytes);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&Path::new("green_with_escapes.bin"))?;
    //file.write_all(&output).unwrap();
    for i in output {
        file.write_all(&i["letter"]).unwrap();
        println!("{}", String::from_utf8(i[0].to_vec()).unwrap());
    }
    Ok(())
}
