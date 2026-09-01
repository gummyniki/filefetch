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
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[arg(long)]
    nocolor: bool,

    #[arg(long, help = "Include files in subdirectories")]
    recursive: bool,

    #[arg(long, help = "List folder sizes (will take longer)")]
    folder_size: bool,
}

fn count_entries_recursively(path: &std::path::Path) -> (usize, usize) {
    let mut folder_count = 0;
    let mut file_count = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                file_count += 1;
            } else if path.is_dir() {
                folder_count += 1;
                let (sub_folders, sub_files) = count_entries_recursively(&path);
                folder_count += sub_folders;
                file_count += sub_files;
            }
        }
    }

    (folder_count, file_count)
}


fn main() {
    let cli = Cli::parse();

    let mut sys = System::new_all();
    sys.refresh_all();

    let folder_size = get_size(env::current_dir().unwrap()).unwrap_or(0);
    let current_dir = env::current_dir().unwrap_or_default();
    let paths = fs::read_dir(&current_dir).unwrap_or_else(|_| fs::read_dir(".").unwrap());
    let paths2 = fs::read_dir(&current_dir).unwrap_or_else(|_| fs::read_dir(".").unwrap());

    let mut fileCount = 0;
    let mut folderCount = 0;

        let (folderCount, fileCount) = if cli.recursive {
        count_entries_recursively(&current_dir)
        } else {
            let mut folder_count = 0;
            let mut file_count = 0;
            for entry2 in paths2 {
                let entry2 = match entry2 {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let md = match metadata(entry2.path()) {
                    Ok(m) => m,
                    Err(_) => continue,
                };
                if md.is_dir() {
                    folder_count += 1;
                } else {
                    file_count += 1;
                }
        }
        (folder_count, file_count)
    };


    if cli.nocolor {
        println!("📁 Current Directory: {}", current_dir.display());
        println!("📦 Folder Size: {:.2} MB", folder_size as f64 / 1024.0 / 1024.0);
        println!("📦 Number of entries: 📁 {} Folders, 📄 {} Files", folderCount, fileCount);
        println!("📄 Files:");
    } else {
        println!("📁 Current Directory: {}", current_dir.display().to_string().magenta());
        println!("📦 Folder Size: {:.2} MB", (folder_size as f64 / 1024.0 / 1024.0).to_string().yellow());
        println!("📦 Number of entries: 📁 {} Folders, 📄 {} Files", folderCount.to_string().cyan(), fileCount.to_string().cyan());
        println!("📄 Files:");
    }

    for entry in paths {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();

        let md = match metadata(&path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        if md.is_dir() {
            if cli.folder_size {
                // Calculate folder size if the flag is set
                let folder_size_bytes = get_size(&path).unwrap_or(0);
                let folder_size_mb = folder_size_bytes as f64 / 1024.0 / 1024.0;

                if cli.nocolor {
                    println!("•  📁 {}       {:.2} MB", path.display(), folder_size_mb);
                } else {
                    println!("•  📁 {}       {:.2} MB", path.display().to_string().blue().bold(), folder_size_mb);
                }
            } else {
                // Old behavior: no size for folders
                if cli.nocolor {
                    println!("•  📁 {}       N/A", path.display());
                } else {
                    println!("•  📁 {}       N/A", path.display().to_string().blue().bold());
                }
            }
        } else {
            // Files: always show size in KB
            let sizekb = (md.len() as f64) / 1024.0;
            if cli.nocolor {
                println!("•  📄 {}       {:.2} KB", path.display(), sizekb);
            } else {
                println!("•  📄 {}       {:.2} KB", path.display().to_string().green(), sizekb);
            }
        }
    }
}
