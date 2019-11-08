use std::path::{Path};
use std::collections::{HashSet};
use std::fs::{DirEntry};

fn main() {
    if let Ok(dir_iter) = RecursiveDirIterator::new(r"C:\Users\nico\source\DevBranches\AgentUI\") {
        let ignore_list = vec![".svn", "obj", "bin", "debug", "release", ".git"].into_iter().collect::<HashSet<&str>>();
        let extension_list = vec!["cpp", "h"].into_iter().collect::<HashSet<&str>>();
        let files = dir_iter.filter(move |file| {
            if let Ok(file) = file {
                let path = file.path();
                return !ignore_entry(&path, &ignore_list) && has_extension(&path, &extension_list);
            }
            return false;
        });

        for entry in files {
            if let Ok(entry) = entry {
                println!("Entry: {:?}", entry);
            }
        }
    }
}

fn ignore_entry<P: AsRef<Path>>(path: P, ignore_list: &HashSet<&str>) -> bool {
    let path = path.as_ref().to_str().expect("Not a valid path");
    for ignore in ignore_list {
        if path.contains(ignore) {
            return true;
        }
    }
    return false;
}

fn get_extension_from_path(path: &str) -> Option<&str> {
    let parts: Vec<&str> = path.split('.').collect();
    if let Some(ext) = parts.last() {
        return Some(ext);
    }
    None
}

fn has_extension<P: AsRef<Path>>(path: P, extensions: &HashSet<&str>) -> bool {
    if let Some(path) = path.as_ref().to_str() {
        if let Some(ext) = get_extension_from_path(path) {
            return extensions.contains(&ext);
        }
    }
    return false;
}

fn is_dir(e: &std::io::Result<DirEntry>) -> bool {
    if let Ok(e) = e {
        if let Ok(meta) = e.metadata() {
            return meta.is_dir();
        }
    }
    return false;
}

struct RecursiveDirIterator {
    stack: Vec<std::fs::ReadDir>
}

impl Iterator for RecursiveDirIterator {
    type Item = std::io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut iter) = self.stack.pop() {
            while let Some(entry) = iter.next() {
                if is_dir(&entry) {
                    let dir = std::fs::read_dir(entry.expect("Dir!").path()).expect("as");
                    self.stack.push(dir);
                    continue;
                }
                self.stack.push(iter);
                return Some(entry);
            }
        }
        None
    }
}

impl RecursiveDirIterator {
    fn new<P: AsRef<Path>>(path: P) -> Result<RecursiveDirIterator, std::io::Error> {
        let dir = std::fs::read_dir(path)?;
        let iter_stack = vec![dir];
        Ok(RecursiveDirIterator {
            stack: iter_stack,
        })
    }
}
