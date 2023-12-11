use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;


pub fn head (path: PathBuf, n: usize) -> Vec<String> {
    let file_open = File::open(&path);
    let file = match file_open {
        Ok(file) => file,
        Err(_err) => panic!()
    };

    let mut res = Vec::new();

    let reader = io::BufReader::new(&file);

    for (i, line) in reader.lines().enumerate().take(n) {
        if let Ok(line) = line {
            // println!("line {}: {}", i, line);
            res.push(line);
        }
    }
    res
}


// pub fn tail (path: Path, n: usize) -> Vec<String> {

// }
