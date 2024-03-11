use regex::bytes::Regex;
use std::io::{ErrorKind, Read};
use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
    process::{Command, Output},
};

#[allow(dead_code)]
fn netcat_command() -> io::Result<Output> {
    Command::new("nc")
        .arg("neo.challenges.cybersecuritychallenge.be")
        .arg("1338")
        .output()
}

pub fn read_bytes_from_file(path: &Path) -> Result<Vec<u8>, io::Error> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let mut buffer = Vec::with_capacity(file.metadata()?.len() as usize);
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn write_bytes_to_file(bytes: &[u8], path: &Path) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().create(true).write(true).open(path)?;
    file.write_all(bytes)?;
    Ok(())
}

pub fn run() -> Result<(), io::Error> {
    //let raw_bytes = netcat_command()?.stdout;
    let raw_bytes = read_bytes_from_file(Path::new("files/raw_bytes.bin"))?;
    let re: Regex = Regex::new(r"\x1B\[\d+;\d+H(\x1B\[(\x39\x32)m)+(?<letter>.)").unwrap();
    let all_captures = re.captures_iter(&raw_bytes);
    let output_bytes = all_captures
        .flat_map(|capture| capture[0].to_vec())
        .collect::<Vec<u8>>();
    write_bytes_to_file(&output_bytes, Path::new("files/output.bin"))?;
    println!("{}", String::from_utf8(output_bytes.to_vec()).map_err(|_| ErrorKind::InvalidData)?);
    Ok(())
}
