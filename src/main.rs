use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::collections::LinkedList;



fn main() -> io::Result<()> {
    let dir_str = "/Users/gipyzarc/sec/dump/com.tencent.xin/Payload/WeChat.app";
    let dir_path = Path::new(dir_str);

    let mut pending_dirs:LinkedList<String> = LinkedList::new();
    if dir_path.is_dir() {
        pending_dirs.push_back(String::from(dir_path.to_str().unwrap()));
    }

    let mut dir_count = 0;
    let mut file_count = 0;


    while !pending_dirs.is_empty() {
        let current_dir = pending_dirs.pop_back().unwrap();

        // read dir
        let entries = fs::read_dir(current_dir)?;
        for entry in entries {
            let entry = entry?;

            let path = entry.path();
            if path.is_dir() {
                dir_count += 1;
                pending_dirs.push_back(String::from(path.to_str().unwrap()));
            } else {
                file_count += 1;


            }
        }
    }

    println!("Directory Count = {}", dir_count);
    println!("File Count = {}", file_count);

    Ok(())
}
