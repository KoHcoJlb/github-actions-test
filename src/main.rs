use walkdir::WalkDir;

fn main() {
    println!("Hello, world!");

    for entry in WalkDir::new("C:\\Program Files (x86)") {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str().map(|s| s.to_lowercase()) {
                if file_name.contains("vcvars") {
                    println!("{:?}", entry.path());
                }
            }
        }
    }
}
