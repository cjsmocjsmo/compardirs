use std::fs;
use std::path::Path;

fn are_folders_equal(folder1: &Path, folder2: &Path) -> Result<bool, Vec<String>> {
    let mut differences = Vec::new();

    // Compare the contents of the folders
    for entry in fs::read_dir(folder1).expect("read_dir call failed") {
        let entry = entry.expect("read_dir entry failed");
        let path1 = entry.path();
        let path2 = folder2.join(entry.file_name());

        if path2.exists() {
            if fs::metadata(&path1).expect("metadata call failed").is_dir() {
                if !are_folders_equal(&path1, &path2)? {
                    differences.push(format!("Directory {} is different", path1.display()));
                }
            } else if fs::read(&path1).expect("read call failed") != fs::read(&path2).expect("read call failed") {
                differences.push(format!("File {} is different", path1.display()));
            }
        } else {
            differences.push(format!("File {} is missing in {}", path1.display(), folder2.display()));
        }
    }

    if differences.is_empty() {
        Ok(true)
    } else {
        Err(differences)
    }
}

fn main() {
    let folder2 = Path::new("/media/pinas/foo1/Music/Music/A");
    let folder1 = Path::new("/media/pinas/foo1/NewMusic/A");

    match are_folders_equal(folder1, folder2) {
        Ok(true) => println!("The folders are identical."),
        Ok(false) => println!("The folders are different."),
        Err(differences) => {
            println!("The folders are different.\n Differences:");
            for difference in differences {
                println!("{}", difference);
            }
        }
    }
}