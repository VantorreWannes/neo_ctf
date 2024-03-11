// src/script.rs

// Importing the Regex type from the 'regex' crate for working with regular expressions on byte arrays.
use regex::bytes::Regex;
// Importing I/O (Input/Output) related types from the standard library.
use std::io::{self, ErrorKind, Read, Write};
// Importing file system operations and process execution utilities from the standard library.
use std::{
    fs::OpenOptions,
    path::Path,
    process::{Command, Output},
};

/// Executes the 'nc' (netcat) command.
/// This function is currently not in use, marked with 'dead_code' to suppress compiler warnings.
/// It serves as an example of how to execute a system command in Rust, similar to using subprocess in Python.
#[allow(dead_code)]
fn netcat_command() -> io::Result<Output> {
    Command::new("nc") // Creating a new system command 'nc' (netcat)
        .arg("neo.challenges.cybersecuritychallenge.be") // Adding first argument
        .arg("1338") // Adding second argument (port)
        .output() // Execute the command and collect its output
}

/// Read bytes from a file at a given path.
/// This is akin to using 'open' in binary mode in Python.
pub fn read_bytes_from_file(path: &Path) -> Result<Vec<u8>, io::Error> {
    let mut file = OpenOptions::new().read(true).open(path)?; // Open the file in read-only mode
    let mut buffer = Vec::with_capacity(file.metadata()?.len() as usize); // Pre-allocate buffer based on file size
    file.read_to_end(&mut buffer)?; // Read file contents into the buffer
    Ok(buffer) // Return the buffer containing the file's bytes
}

/// Write bytes to a file at a given path.
/// The function takes a byte slice (similar to Python's bytes) and a path reference.
pub fn write_bytes_to_file(bytes: &[u8], path: &Path) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().create(true).write(true).open(path)?; // Open or create the file for writing
    file.write_all(bytes)?; // Write all bytes to the file
    Ok(()) // Indicate success with an empty tuple
}

/// The main functionality of the script, which is called by the main function in `main.rs`.
pub fn run() -> Result<(), io::Error> {
    let raw_bytes = read_bytes_from_file(Path::new("files/raw_bytes.bin"))?; // Reading raw bytes from file
    let re: Regex = Regex::new(r"\x1B\[\d+;\d+H(\x1B\[(\x39\x32)m)+(?<letter>.)").unwrap(); // Creating regex pattern
    let all_captures = re.captures_iter(&raw_bytes); // Finding all matches of regex in raw bytes
    let output_bytes = all_captures
        .flat_map(|capture| capture[0].to_vec())
        .collect::<Vec<u8>>(); // Collecting matched bytes into a vector
    write_bytes_to_file(&output_bytes, Path::new("files/output.bin"))?; // Writing processed bytes to a file
    println!("{}", String::from_utf8(output_bytes.to_vec()).map_err(|_| ErrorKind::InvalidData)?); // Print output as UTF-8 string
    Ok(()) // Indicate successful execution
}

// Unit tests in Rust are written using the #[cfg(test)] attribute and the #[test] attribute on test functions.

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module to use in tests
    use std::fs;

    // A test function to verify that `write_bytes_to_file` correctly writes data to a file.
    #[test]
    fn test_write_bytes_to_file() {
        let test_path = Path::new("test_output.bin");
        let bytes_to_write = b"Hello, Rust!"; // Byte string, similar to Python's b"string" syntax

        // Write the bytes to the file and assert that it didn't return an error
        assert!(write_bytes_to_file(bytes_to_write, test_path).is_ok());

        // Read the file back and assert the contents are as expected
        let read_bytes = fs::read(test_path).expect("Failed to read file");
        assert_eq!(bytes_to_write.to_vec(), read_bytes, "File contents are not equal to the bytes written.");

        // Clean up: remove the file after the test
        fs::remove_file(test_path).expect("Failed to remove test file");
    }

    // You could add more tests here for other functions like `read_bytes_from_file` and `run`
}
