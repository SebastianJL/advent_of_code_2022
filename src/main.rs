use std::{cell::RefCell, error::Error, rc::Rc, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let mut fs = parse_input(&input);
    fs.print();

    let total_space = 70_000_000;
    let update_size = 30_000_000;

    let available_space = total_space - fs.disk_usage();
    let need_to_free = update_size - available_space;

    let dir_sizes = fs.gather_dir_sizes();
    let small_dirs = dir_sizes.iter().filter(|(_, size)| *size >= need_to_free);
    let smallest_possible_dir = small_dirs.map(|(_, size)| size).min().unwrap();
    dbg!(smallest_possible_dir);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn parse_input(input: &str) -> FileSystem {
    let root = Rc::new(RefCell::new(Directory::new("/")));
    let mut current_dir = Rc::clone(&root);
    let mut iter = input.lines().peekable();
    iter.next().unwrap();
    while let Some(line) = iter.next() {
        if line.starts_with("$ cd ..") {
            let parent = Rc::clone(current_dir.borrow().parent.as_ref().unwrap());
            current_dir = parent;
        } else if let Some(name) = line.strip_prefix("$ cd ") {
            let mut new_dir = Directory::new(name);
            new_dir.parent = Some(Rc::clone(&current_dir));
            let new_dir = Rc::new(RefCell::new(new_dir));
            current_dir.borrow_mut().dirs.push(Rc::clone(&new_dir));
            current_dir = new_dir;
        } else if line.starts_with("$ ls") {
            while let Some(next_line) = iter.peek() {
                if next_line.starts_with('$') {
                    break;
                } else if next_line.starts_with("dir") {
                    iter.next().unwrap();
                } else {
                    let line = iter.next().unwrap();
                    let (size, name) = line.split_once(' ').unwrap();
                    let size: u32 = size.parse().unwrap();
                    current_dir.borrow_mut().files.push(File::new(name, size));
                }
            }
        }
    }

    FileSystem {
        root: Rc::clone(&root),
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: &str, size: u32) -> Self {
        File {
            name: name.to_owned(),
            size,
        }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    dirs: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
    size: Option<u32>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: name.to_owned(),
            parent: None,
            dirs: vec![],
            files: vec![],
            size: None,
        }
    }

    fn print(&self, depth: usize) {
        println!("{:>pad$}- {:} (dir)", "", self.name, pad = 2 * depth);
        for dir in &self.dirs {
            dir.borrow().print(depth + 1);
        }
        for file in &self.files {
            println!(
                "{:>pad$}- {:} (file, size={})",
                "",
                file.name,
                file.size,
                pad = 2 * (depth + 1),
            );
        }
    }

    fn size(&mut self) -> u32 {
        if let Some(size) = self.size {
            return size;
        }
        let mut size = self.files.iter().map(|file| file.size).sum();
        size += self
            .dirs
            .iter()
            .map(|dir| dir.borrow_mut().size())
            .sum::<u32>();
        self.size = Some(size);
        size
    }

    fn append_dir(&mut self, dirs: &mut Vec<(String, u32)>) {
        dirs.push((self.name.clone(), self.size()));
        for dir in self.dirs.iter_mut() {
            dir.borrow_mut().append_dir(dirs);
        }
    }
}

struct FileSystem {
    root: Rc<RefCell<Directory>>,
}

impl FileSystem {
    fn print(&self) {
        self.root.borrow().print(0);
    }

    fn disk_usage(&mut self) -> u32 {
        self.root.borrow_mut().size()
    }

    fn gather_dir_sizes(&mut self) -> Vec<(String, u32)> {
        let mut dirs = vec![];
        self.root.borrow_mut().append_dir(&mut dirs);

        dirs
    }
}