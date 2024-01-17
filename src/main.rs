use std::fs;
use std::path::PathBuf;

fn main() {
    // Define the source file path
    let source_file_path = "pa_detail.parquet";

    // Define the base directory for the new locations
    let base_directory = "part_account";

    // Create directories and duplicate the file
    for id in 1..=100 {
        // Create the directory path for the current id
        let directory_path = format!("{}/{}", base_directory, id);

        // Create the directory if it doesn't exist
        if let Err(err) = fs::create_dir_all(&directory_path) {
            eprintln!("Error creating directory {}: {}", &directory_path, err);
            continue; // Skip to the next id if there's an error
        }

        // Create a new path for the duplicated file
        let new_file_path = PathBuf::from(&directory_path).join("file.parquet");

        // Copy the source file to the new location
        if let Err(err) = fs::copy(&source_file_path, &new_file_path) {
            eprintln!(
                "Error copying file to {}: {}",
                &new_file_path.display(),
                err
            );
            continue; // Skip to the next id if there's an error
        }

        println!("Duplicated file to: {}", &new_file_path.display());
    }

    println!("Duplication completed for 100 locations.");
}
