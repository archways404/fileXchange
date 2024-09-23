use rfd::FileDialog;
use std::fs::{self, File, create_dir_all};
use std::io::{Read, Write};
use base64;
use std::path::{Path, PathBuf};

// Main function
fn main() {
    // Select files and create the output directory
    let files = select_files();
    let out_dir = PathBuf::from("out");
    
    // Ensure output directory exists
    if let Err(e) = create_output_dir(&out_dir) {
        println!("Failed to create the output directory. Error: {}", e);
        return;
    }

    // Process each selected file
    if let Some(paths) = files {
        for path in paths {
            if let Err(e) = process_file(&path, &out_dir) {
                println!("Failed to process file: {:?}. Error: {}", path.display(), e);
            }
        }
    } else {
        println!("No files were selected.");
    }
}

// Selects files via file dialog
fn select_files() -> Option<Vec<PathBuf>> {
    FileDialog::new()
        .set_title("Select Files")
        .pick_files()
}

// Ensures that the output directory exists
fn create_output_dir(out_dir: &Path) -> std::io::Result<()> {
    create_dir_all(out_dir)
}

// Processes the file by reading it in chunks, encoding each chunk in Base64, and saving it
fn process_file(path: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file
    let mut file = File::open(path)?;

    // Define chunk size (in bytes)
    let chunk_size = 10;

    // Get file stem and extension safely
    let file_stem = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown_file");
    let file_extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("txt");

    let mut buffer = vec![0u8; chunk_size];  // Temporary buffer for reading chunks
    let mut chunk_index = 1;  // Track chunk number

    // Read the file chunk by chunk
    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;  // Exit the loop when we've read the entire file
        }

        // Only take the bytes that were read in case it's less than the buffer size
        let chunk_data = &buffer[..bytes_read];

        // Encode the chunk to Base64
        let base64_data = encode_base64(chunk_data);

        // Save the Base64 chunk
        save_chunk(&base64_data, file_stem, file_extension, out_dir, chunk_index)?;

        chunk_index += 1;  // Move to the next chunk
    }

    Ok(())
}

// Encodes a chunk of file data to Base64
fn encode_base64(contents: &[u8]) -> String {
    base64::encode(contents)
}

// Saves a Base64-encoded chunk as a file
fn save_chunk(base64_data: &str, file_stem: &str, file_extension: &str, out_dir: &Path, chunk_index: usize) -> Result<(), std::io::Error> {
    // Construct the chunk filename and path in the 'out' directory
    let chunk_filename = format!(
        "{}_chunk_{}.{}",
        file_stem,
        chunk_index,  // Number the chunks sequentially
        file_extension
    );
    let chunk_path = out_dir.join(chunk_filename);

    // Save the Base64 data to the chunk file
    let mut chunk_file = File::create(&chunk_path)?;
    chunk_file.write_all(base64_data.as_bytes())?;

    println!("Chunk saved: {:?}", chunk_path.display());

    Ok(())
}
