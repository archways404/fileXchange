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

// Reads the file and processes it (encodes to Base64, chunks, and saves chunks)
fn process_file(path: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file contents
    let file_contents = read_file_contents(path)?;
    
    // Convert file contents to Base64
    let base64_data = encode_base64(&file_contents);

    // Print file size information
    print_file_size(path, &file_contents, &base64_data);

    // Chunk the Base64 data and save each chunk
    let file_stem = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown_file");
    let file_extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("txt");

    save_chunks(&base64_data, file_stem, file_extension, out_dir)?;

    Ok(())
}

// Reads the contents of the file into a Vec<u8>
fn read_file_contents(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

// Encodes the file contents to Base64
fn encode_base64(contents: &[u8]) -> String {
    base64::encode(contents)
}

// Prints the original file size and the Base64 size
fn print_file_size(path: &Path, original: &[u8], base64_encoded: &str) {
    let original_size = original.len();
    let base64_size = base64_encoded.len();
    println!(
        "File: {:?}, Original Size: {} bytes, Base64 Size: {} bytes",
        path.display(),
        original_size,
        base64_size
    );
}

// Chunks Base64 data into 10-byte chunks and saves them as files
fn save_chunks(base64_data: &str, file_stem: &str, file_extension: &str, out_dir: &Path) -> Result<(), std::io::Error> {
    let chunk_size = 10;
    let chunks = base64_data.as_bytes().chunks(chunk_size).enumerate();

    for (i, chunk) in chunks {
        let chunk_filename = format!(
            "{}_chunk_{}.{}",
            file_stem,
            i + 1,  // Chunk numbering starts from 1
            file_extension
        );
        let chunk_path = out_dir.join(chunk_filename);

        let mut chunk_file = File::create(&chunk_path)?;
        chunk_file.write_all(chunk)?;

        println!("Chunk saved: {:?}", chunk_path.display());
    }
    Ok(())
}
