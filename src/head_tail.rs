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

    for (_i, line) in reader.lines().enumerate().take(n) {
        if let Ok(line) = line {
            // println!("line {}: {}", i, line);
            res.push(line.clone());
        }
    }
    res
}

pub fn tail(path: PathBuf, n: usize) -> Vec<String> {
    let file_open = File::open(&path);
    let file = match file_open {
        Ok(file) => file,
        Err(_err) => panic!()
    };

    let mut res = Vec::new();

    let reader = io::BufReader::new(&file);

    let all_lines: Vec<_> = reader.lines().collect();

    let start_index = if all_lines.len() > n {
        all_lines.len() - n
    } else {
        0
    };

    for (_i, line) in all_lines.iter().skip(start_index).enumerate() {
        if let Ok(line) = line {
            // println!("line {}: {}", i, line);
            res.push(line.clone());
        }
    }

    res
}