// #![allow(unused)]
mod argc;
mod error;

use std::{error::Error, path::PathBuf, process};
use clap::Parser;
use file_size::fit_4;
use walkdir::{DirEntry, WalkDir};
use argc::Args as clap_args;
use error::AppError;

const DIR: &str = "./";

struct Entry {
    path: PathBuf,
    size: u64,
}

// command: cargo watch -q -c -x 'run -q -- -n 3'     (to use auto-run)
fn main() {
    let num = parse_args().unwrap_or_else(|err| {
        eprintln!("Error: {:?}", err.to_string());
        process::exit(1);
    });


    run(num).unwrap_or_else(|err| {
        println!("Error: {:?}", err);
    })
}

fn is_skipable(entry: &DirEntry) -> bool {
    entry.path()
        .to_str()
        .map(|s| !s.contains(".git"))
        .unwrap_or(false)
}

fn parse_args() -> Result<usize, AppError> {
    let args = clap_args::try_parse();
    
     match args {
        Ok(n) => return Ok(n.nums),
        Err(_) => return Err(AppError::InvalidNumberOfFiles("not a number".to_string())),
    };    
    
}

fn run(nums: usize) -> Result<(), Box<dyn Error>> {
    let mut total_size: u64 = 0;
    let mut total_files: usize = 0;
    let mut tops: Vec<Entry> = Vec::with_capacity(nums + 1);
    let mut min_of_tops: u64 = 0;

    for ent in WalkDir::new(DIR).into_iter().filter_entry(|e| is_skipable(e) ) {
        let entry = ent?; 
        let path = entry.path();
        if path.is_file() && !path.is_symlink() {
            total_files += 1;

            let size = entry.metadata()?.len();
            total_size += size;

            if min_of_tops < size {
                tops.push(Entry {
                    path: path.to_path_buf(),
                    size,
                });

                tops.sort_by(|a, b| a.size.cmp(&b.size));

                if tops.len() > nums {
                    tops.pop();
                }
                min_of_tops = tops.last().map(|e| e.size).unwrap_or(0);
            }
        }
        
    }
    
    println!("Number of files : {} and total size: {}", total_files, fit_4(total_size));
    println!("Top {} biggest files", tops.len());

    for Entry {path, size} in tops.iter() {
        println!("{:<5} - {}", fit_4(*size), path.to_string_lossy()) 
    }

    Ok(())
}
