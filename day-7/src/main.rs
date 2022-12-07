extern crate core;

use std::{env, fs};
use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::path::PathBuf;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide an input for the program");

    let file_content = fs::read_to_string(file_path).expect("Could not read file");

    // Still use .lines to get rid of any \n
    let mut input = file_content.lines();
    let mut virtual_fs = VirtualFS::new();

    for line in input {
        if line.starts_with('$') {
            let mut parts = line.split(' ');
            parts.next();

            if parts.next().unwrap() == "cd" {
                virtual_fs.cd(parts.next().unwrap());
            }
        } else if !line.starts_with("dir") {
            let mut parts = line.split(' ');
            let size = parts.next().unwrap().parse().unwrap();
            let name = parts.next().unwrap();

            virtual_fs.touch(name, size);
        }
    }

    let total: u32 = virtual_fs.get_dir_sizes()
        .iter()
        .filter(|(_, size)| size < &100000_u32)
        .map(|(_, size)| size)
        .sum();

    println!("{:?}", total);
}

#[derive(Debug)]
struct VirtualFS {
    dir_map: BTreeMap<String, VirtualDir>,
    history: Vec<String>,
}

impl VirtualFS {
    pub fn new() -> Self {
        let mut instance = Self {
            dir_map: Default::default(),
            history: vec!["/".to_string()],
        };

        instance.dir_map.insert("/".to_string(), VirtualDir::default());

        instance
    }

    fn get_current_dir(&mut self) -> &mut VirtualDir {
        self.dir_map.get_mut(&*self.cwd()).unwrap()
    }

    pub fn cwd(&self) -> String {
        self.history.last().unwrap().to_string()
    }

    pub fn cd(&mut self, path: &str) {
        match path {
            "/" => {
                self.history.clear();
                self.history.push("/".to_string());
            },

            ".." => {
                self.history.pop();
            },

            _ => {
                let new_path = push_dir(&self.cwd(), &path.to_string());

                self.mkdir(new_path.to_string());
                self.history.push(new_path);
            }
        }
    }

    pub fn mkdir(&mut self, path: String) {
        self.get_current_dir().dirs.push(path.to_string());
        self.dir_map.entry(path.to_string())
            .or_default();
    }

    pub fn touch(&mut self, name: &str, size: u32) {
        let current_path = self.cwd();
        let current_dir = self.dir_map
            .get_mut(&*current_path)
            .unwrap();

        current_dir.files.push(VirtualFile {
            name: name.to_string(),
            size,
        })
    }

    pub fn get_dir_size(&self, path: String) -> u32 {
        let dir = self.dir_map.get(&*path)
            .unwrap();

        let mut total = dir.get_total_content_size();
        for path in &dir.dirs {
            total += self.get_dir_size(path.to_string());
        }

        total
    }

    pub fn get_dir_sizes(&self) -> Vec<(String, u32)> {
        let mut paths = vec![];

        for (path, _) in &self.dir_map {
            let size = self.get_dir_size(path.to_string());
            paths.push((path.to_string(), size));
        }

        paths
    }
}

#[derive(Debug, Default)]
struct VirtualDir {
    dirs: Vec<String>,
    files: Vec<VirtualFile>,
}

impl VirtualDir {
    pub fn get_total_content_size(&self) -> u32 {
        self.files.iter()
            .map(|file| file.size)
            .sum()
    }
}

#[derive(Debug)]
struct VirtualFile {
    name: String,
    size: u32,
}

fn push_dir(current_path: &String, dir: &String) -> String {
    let mut new_path = current_path.clone();

    if !new_path.ends_with('/') {
        new_path.push('/');
    }

    new_path + dir
}

fn pop_dir(path: &String) -> String {
    let mut parts: Vec<&str> = path.split('/')
        .collect();

    let new_index = parts.len() - 1;
    parts.drain(new_index..);

    parts.join("/")
}
