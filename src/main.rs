use std::fs;
use std::path::Path;

fn print_tree(path: &Path, prefix: &str) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
        entries.sort_by_key(|e| e.path());

        for (i, entry) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1;
            let entry_path = entry.path();
            let entry_name = entry_path.file_name().unwrap().to_string_lossy();

            println!("{}{} {}", prefix, if is_last { "└──" } else { "├──" }, entry_name);

            if entry_path.is_dir() {
                let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                print_tree(&entry_path, &new_prefix);
            }
        }
    }
}

fn main() {
    let root = Path::new("/home/n3tw4lk3r/programs/sandbox/");
    println!("{}", root.display());
    print_tree(root, "");
}
