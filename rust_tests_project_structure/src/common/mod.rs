pub(crate) mod print_file_contents;

pub fn print_bytes(content_bytes: &[u8]) {
    // Convert the byte slice to a &str
    if let Ok(content_str) = std::str::from_utf8(content_bytes) {
        println!("{}", content_str);
    } else {
        eprintln!("The file is not valid UTF-8.");
    }
}
