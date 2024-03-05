use std::fs::File;
use std::io::{BufReader, BufRead};

const BUFFER_SIZE: usize = 2*8192;

#[tauri::command]
pub fn read_file(filename: &str) -> tauri::Result<String> {
    let contents = std::fs::read_to_string(filename)?;
    Ok(contents)
}

pub fn do_something_with_buffer(buffer: &[u8]) {
    // Do something with the buffer
}
#[tauri::command]
pub fn read_file_bytes(filename: &str) -> tauri::Result<String> {
    let file: File = File::open(filename)?;
    let mut reader: BufReader<File> = BufReader::with_capacity(BUFFER_SIZE, file);

    loop {
        let buffer: &[u8] = reader.fill_buf()?;
        let buffer_length: usize = buffer.len();

        if buffer_length == 0 {
            break;
        }

        do_something_with_buffer(buffer);

        reader.consume(buffer_length);
    }

    let contents = std::fs::read_to_string(filename)?;
    Ok(contents)
}