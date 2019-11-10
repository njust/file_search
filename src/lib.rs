use std::path::{Path};
use std::collections::{HashSet};
use std::fs::{DirEntry};
use std::process::Stdio;

pub fn open_file(file_path: &String) {
    if cfg!(target_os = "windows") {
        if let Err(err) = std::process::Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg(file_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn() {
            println!("Could not open file: {:?}", err);
        }
    }else {
        if let Err(err) = std::process::Command::new("/usr/bin/xdg-open")
            .arg(file_path)
            .spawn() {
            println!("Could not open file: {:?}", err);
        }
    }
}

pub fn ignore_entry<P: AsRef<Path>>(path: P, ignore_list: &HashSet<&str>) -> bool {
    let path = path.as_ref().to_str().expect("Not a valid path");
    for ignore in ignore_list {
        if path.contains(ignore) {
            return true;
        }
    }
    return false;
}

pub fn get_extension_from_path(path: &str) -> Option<&str> {
    let parts: Vec<&str> = path.split('.').collect();
    if let Some(ext) = parts.last() {
        return Some(ext);
    }
    None
}

pub fn has_extension<P: AsRef<Path>>(path: P, extensions: &HashSet<&str>) -> bool {
    if let Some(path) = path.as_ref().to_str() {
        if let Some(ext) = get_extension_from_path(path) {
            return extensions.contains(&ext);
        }
    }
    return false;
}

pub fn is_dir(e: &std::io::Result<DirEntry>) -> bool {
    if let Ok(e) = e {
        if let Ok(meta) = e.metadata() {
            return meta.is_dir();
        }
    }
    return false;
}

pub struct RecursiveDirIterator {
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
    pub fn new<P: AsRef<Path>>(path: P) -> Result<RecursiveDirIterator, std::io::Error> {
        let dir = std::fs::read_dir(path)?;
        let iter_stack = vec![dir];
        Ok(RecursiveDirIterator {
            stack: iter_stack,
        })
    }
}
