use sysinfo::{System};
extern crate fs_extra;
use fs_extra::dir::get_size;
use std::env;
use std::fs;
use std::fs::metadata;
use colored::*;
use clap::Parser;

#[derive(Parser)]
#[command(name = "filefetch")]
#[command(about = "A folder info fetcher", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[arg(long)]
    nocolor: bool,

    #[arg(long, help = "Include files in subdirectories")]
    recursive: bool,
}


fn count_files_recursively(path: &std::path::Path) -> usize {
    let mut count = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                count += 1;
            } else if path.is_dir() {
                count += count_files_recursively(&path);
            }
        }
    }

    count
}



fn main() {

    let cli = Cli::parse();

    let mut sys = System::new_all();
    sys.refresh_all();

    let folder_size = get_size(env::current_dir().unwrap()).unwrap();
    let current_dir = env::current_dir().unwrap();
    let paths = fs::read_dir(&current_dir).unwrap();

    let file_count = if cli.recursive {
        count_files_recursively(&current_dir)
    } else {
        fs::read_dir(&current_dir)
            .unwrap()
            .count()
    };


    if cli.nocolor {
        println!("📁 Current Directory: {}", env::current_dir().unwrap().display());
        println!("📦 Folder Size: {:.2} MB", folder_size as f64 / 1024.0 / 1024.0);
        println!("📦 Number of entries: {}", file_count);
        println!("📄 Files:");
    } else {
        println!("📁 Current Directory: {}", env::current_dir().unwrap().display().to_string().magenta());
        println!("📦 Folder Size: {:.2} MB", (folder_size as f64 / 1024.0 / 1024.0).to_string().yellow());
        println!("📦 Number of entries: {}", file_count.to_string().cyan());
        println!("📄 Files:");
    }


    

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();
        
        let md = metadata(&path).unwrap();

        if cli.nocolor {
            if md.is_dir() {
            println!("•  📁 {}", path.display());
        } else {
            println!("•  📄 {}", path.display());
        }
        } else {
            if md.is_dir() {
            println!("•  📁 {}", path.display().to_string().blue().bold());
            } else {
                println!("•  📄 {}", path.display().to_string().green());
            }
        }

        
    }

}
