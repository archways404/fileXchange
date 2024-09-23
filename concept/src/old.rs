use rfd::FileDialog;
use std::fs::{self, File, create_dir_all};
use std::io::{Read, Write};
use base64;
use std::path::PathBuf;

fn main() {
    // Open the file picker dialog and allow the user to select multiple files
    let files = FileDialog::new()
        .set_title("Select Files")
        .pick_files();

    // Define the output directory
    let out_dir = PathBuf::from("out");

    // Create the output directory if it doesn't exist
    if let Err(e) = create_dir_all(&out_dir) {
        println!("Failed to create the output directory. Error: {}", e);
        return;
    }

    // Check if the user has selected files
    if let Some(paths) = files {
        for path in paths {
            // Read the file contents
            match fs::File::open(&path) {
                Ok(mut file) => {
                    let mut buffer = Vec::new();
                    match file.read_to_end(&mut buffer) {
                        Ok(_) => {
                            // Convert the file contents to Base64
                            let base64_data = base64::encode(&buffer);

                            // Print the original file size and the Base64 size
                            let original_size = buffer.len();
                            let base64_size = base64_data.len();
                            println!(
                                "File: {:?}, Original Size: {} bytes, Base64 Size: {} bytes",
                                path.display(),
                                original_size,
                                base64_size
                            );

                            // Chunk the Base64 data into 10-byte chunks
                            let chunk_size = 10;
                            let chunks = base64_data
                                .as_bytes()
                                .chunks(chunk_size)
                                .enumerate();

                            // Get the original file stem and extension safely
                            let file_stem = path.file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("unknown_file");  // Default if no stem
                            
                            let file_extension = path.extension()
                                .and_then(|e| e.to_str())
                                .unwrap_or("txt");  // Default to 'txt' if no extension

                            for (i, chunk) in chunks {
                                // Construct the chunk filename and path in the 'out' directory
                                let chunk_filename = format!(
                                    "{}_chunk_{}.{}",
                                    file_stem,
                                    i + 1,  // Chunk numbering starts from 1
                                    file_extension
                                );
                                let chunk_path = out_dir.join(chunk_filename);

                                // Save each chunk as a separate file in the 'out' directory
                                match File::create(&chunk_path) {
                                    Ok(mut chunk_file) => {
                                        // Write the chunk data to the file
                                        if let Err(e) = chunk_file.write_all(chunk) {
                                            println!("Failed to write chunk: {}. Error: {}", chunk_path.display(), e);
                                        } else {
                                            println!("Chunk saved: {:?}", chunk_path.display());
                                        }
                                    }
                                    Err(e) => {
                                        println!("Failed to create chunk file: {:?}. Error: {}", chunk_path.display(), e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to read the file: {:?}. Error: {}", path.display(), e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to open the file: {:?}. Error: {}", path.display(), e);
                }
            }
        }
    } else {
        println!("No files were selected.");
    }
}
