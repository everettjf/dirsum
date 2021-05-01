use filesize::PathExt;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
struct FileExtensionSummary {
    extension: String,
    count: u32,
    total_size: u64,
}

#[derive(Debug)]
struct FileBasicInfo {
    name: String,
    size: u64,
    path: String,
}
#[derive(Debug)]
struct DirBasicInfo {
    name: String,
    size: u64,
    path: String,
}

fn compute_directory_size(dir_str: &str) -> u64 {
    let dir_path = Path::new(dir_str);
    let mut pending_dirs: LinkedList<String> = LinkedList::new();
    if dir_path.is_dir() {
        pending_dirs.push_back(String::from(dir_path.to_str().unwrap()));
    }
    let mut total_file_size: u64 = 0;

    while !pending_dirs.is_empty() {
        let current_dir = pending_dirs.pop_back().unwrap();

        // read dir
        let entries = fs::read_dir(current_dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();

            let path = entry.path();
            if path.is_dir() {
                pending_dirs.push_back(String::from(path.to_str().unwrap()));
            } else {
                let file_size = path.as_path().size_on_disk().unwrap_or_default();
                total_file_size += file_size;
            }
        }
    }

    total_file_size
}



pub fn parse(dir_str: &str) -> io::Result<()> {
  let dir_path = Path::new(dir_str);

  let mut pending_dirs: LinkedList<String> = LinkedList::new();
  if dir_path.is_dir() {
      pending_dirs.push_back(String::from(dir_path.to_str().unwrap()));
  }

  let mut dir_count: u32 = 0;
  let mut file_count: u32 = 0;
  let mut total_file_size: u64 = 0;
  let mut sorted_file_extension_count = Vec::new();
  let mut framework_items = Vec::new();
  let mut plugin_items = Vec::new();
  let mut files_without_extensions = Vec::new();
  let mut top_large_files = Vec::new();

  let mut file_extension_map: HashMap<String, FileExtensionSummary> = HashMap::new();
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
              let file_ext = path
                  .extension()
                  .and_then(OsStr::to_str)
                  .unwrap_or_else(|| "");

              let file_size = path.as_path().size_on_disk().unwrap_or_default();
              total_file_size += file_size;

              let summary = file_extension_map.entry(String::from(file_ext)).or_insert(
                  FileExtensionSummary {
                      extension: String::from(file_ext),
                      count: 0,
                      total_size: 0,
                  },
              );
              summary.count += 1;
              summary.total_size += file_size;

              let relative_path = path
                  .to_str()
                  .unwrap_or_default()
                  .strip_prefix(dir_str)
                  .unwrap_or_default();

              if file_ext.is_empty() {
                  files_without_extensions.push(FileBasicInfo {
                      name: String::from(
                          path.file_name().and_then(OsStr::to_str).unwrap_or_default(),
                      ),
                      size: file_size,
                      path: String::from(relative_path),
                  });
              }

              // top large files
              top_large_files.push(FileBasicInfo {
                  name: String::from(
                      path.file_name().and_then(OsStr::to_str).unwrap_or_default(),
                  ),
                  size: file_size,
                  path: String::from(relative_path),
              })
          }
      }
  }
  for (_, summary) in file_extension_map.into_iter() {
      sorted_file_extension_count.push(summary);
  }
  sorted_file_extension_count.sort_by(|a, b| b.count.partial_cmp(&a.count).unwrap());

  // frameworks
  let framework_path = dir_path.join("Frameworks");
  if framework_path.exists() && framework_path.is_dir() {
      for entry in fs::read_dir(framework_path)? {
          let entry = entry?;
          let path = entry.path();

          if !path.is_dir() {
              continue;
          }

          let dir_name = entry.file_name().into_string().unwrap_or_default();
          let dir_size = compute_directory_size(path.to_str().unwrap());
          let relative_path = path
              .to_str()
              .unwrap_or_default()
              .strip_prefix(dir_str)
              .unwrap_or_default();
          framework_items.push(DirBasicInfo {
              name: dir_name,
              size: dir_size,
              path: String::from(relative_path),
          });
      }
  }

  // plugins
  let plugin_path = dir_path.join("Plugins");
  if plugin_path.exists() && plugin_path.is_dir() {
      for entry in fs::read_dir(plugin_path)? {
          let entry = entry?;
          let path = entry.path();

          if !path.is_dir() {
              continue;
          }

          let dir_name = entry.file_name().into_string().unwrap_or_default();
          let dir_size = compute_directory_size(path.to_str().unwrap());
          let relative_path = path
              .to_str()
              .unwrap_or_default()
              .strip_prefix(dir_str)
              .unwrap_or_default();
          plugin_items.push(DirBasicInfo {
              name: dir_name,
              size: dir_size,
              path: String::from(relative_path),
          });
      }
  }


  // top large files
  top_large_files.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());
  top_large_files.drain(10..);

  println!("Directory Count = {}", dir_count);
  println!("File Count = {}", file_count);
  println!("Total File Size = {}", total_file_size);
  println!("File Extensions Count = {:#?}", sorted_file_extension_count);
  println!("Files Without Extension = {:#?}", files_without_extensions);
  println!("Top Large Files = {:#?}", top_large_files);
  if !framework_items.is_empty() {
      println!("Framework Items = {:#?}", framework_items);
  }
  if !plugin_items.is_empty() {
      println!("Plugin Items = {:#?}", plugin_items);
  }

  Ok(())
}
