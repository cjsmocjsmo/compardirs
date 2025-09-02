use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_jpgs(dir: &Path) -> Vec<PathBuf> {
    let mut jpgs = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            println!("Processing: {}", path.display());
            if path.is_dir() {
                jpgs.extend(collect_jpgs(&path));
            } else if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("jpg") {
                    jpgs.push(path);
                }
            }
        }
    }
    jpgs
}

fn main() {
    let dira = Path::new("/media/whitepi/ATree/RustMasterPics"); // <-- set your path
    let dirb = Path::new("media/whitepi/ATree/Clean"); // <-- set your path

    let jpgs_a = collect_jpgs(dira);
    let jpgs_b = collect_jpgs(dirb);

    // Build a set of file names in dirb
    let names_b: HashSet<_> = jpgs_b.iter()
        .filter_map(|p| p.file_name().map(|n| n.to_owned()))
        .collect();

    // Find jpgs in dira not found in dirb (by file name)
    let mut not_in_b = Vec::new();
    for path in &jpgs_a {
        if let Some(name) = path.file_name() {
            if !names_b.contains(name) {
                not_in_b.push(path);
            }
        }
    }

    println!("Images in {} not found in {}:", dira.display(), dirb.display());
    for path in not_in_b {
        println!("{}", path.display());
    }
}