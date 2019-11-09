use std::collections::{HashSet};
use search_test::{RecursiveDirIterator, ignore_entry, has_extension};


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
