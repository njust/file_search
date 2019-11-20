use std::path::{Path};
use std::collections::{HashSet};
use std::fs::{DirEntry};
use std::process::Stdio;

mod widget;
use iced::{
    button, text::HorizontalAlignment, Background,  Button, Color, Text
};

pub use widget::tab;

#[derive(Debug, Clone)]
pub struct Search {
    pub query: String,
    pub path: String,
}

#[derive(Debug, Clone, Default)]
pub struct ResultItem {
    pub path: String,
}

impl Search {
    pub fn new(query: String, path: String) -> Self {
        Self {
            query,
            path
        }
    }
    pub async fn run(self) -> Result<Vec<ResultItem>, SearchError> {
        let mut res = vec![];
        let iter = RecursiveDirIterator::new(&self.path).map_err(|_|SearchError::General)?;
        let search = self.query.to_lowercase();
        for entry in iter {
            if let Ok(entry) = entry {
                let path = entry.path();
                let path = path.to_str().ok_or(SearchError::InvalidPath)?.to_owned();
                if path.to_lowercase().contains(&search) {
                    res.push(ResultItem {
                        path
                    });
                }
            }
        }
        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub enum SearchError {
    General,
    InvalidPath,
}

#[derive(Debug, Clone)]
pub enum SearchMessage {
    InputChanged(String),
    SearchPressed(String),
    ItemSelected(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(i16),
    Inc,
    SearchResult(Result<Vec<ResultItem>, SearchError>),
    SearchMsg(SearchMessage)
}

pub fn create_button<'a, T>(label: &str, state: &'a mut button::State) -> Button<'a, T> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center))
        .border_radius(4)
        .background(Background::Color(Color{
            r: 0.0, g: 0.0, b: 0.2, a: 0.5
        }))
        .padding(4)
}

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

pub fn is_dir(e: &DirEntry) -> bool {
    if let Ok(meta) = e.metadata() {
        return meta.is_dir();
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
            while let Some(Ok(entry)) = iter.next() {
                if is_dir(&entry) {
                    if let Ok(dir) = std::fs::read_dir(entry.path()) {
                        self.stack.push(dir);
                    }
                    continue;
                }
                self.stack.push(iter);
                return Some(Ok(entry));
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
