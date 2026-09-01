use sysinfo::{System};
extern crate fs_extra;
use fs_extra::dir::get_size;
use std::env;
use std::fs;
use std::fs::metadata;
use colored::*;
use clap::{Parser, ValueEnum};

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

    #[arg(long, value_enum, default_value = "none", help = "Sort entries by name or size")]
    sort: SortOrder,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SortOrder {
    None,
    NameAsc,
    NameDesc,
    SizeAsc,
    SizeDesc,
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

        let (folder_count, file_count) = if cli.recursive {
        count_entries_recursively(&current_dir)
    } else {
        let mut folder_count = 0;
        let mut file_count = 0;
        for entry2 in fs::read_dir(&current_dir).unwrap_or_else(|_| fs::read_dir(".").unwrap()) {
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
        println!("📦 Number of entries: 📁 {} Folders, 📄 {} Files", folder_count, file_count);
        println!("📄 Files:");
    } else {
        println!("📁 Current Directory: {}", current_dir.display().to_string().magenta());
        println!("📦 Folder Size: {:.2} MB", (folder_size as f64 / 1024.0 / 1024.0).to_string().yellow());
        println!("📦 Number of entries: 📁 {} Folders, 📄 {} Files", folder_count.to_string().cyan(), file_count.to_string().cyan());
        println!("📄 Files:");
    }

    let mut entries: Vec<(std::path::PathBuf, u64, bool)> = Vec::new();

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

        let size = if md.is_dir() {
            if cli.folder_size {
                get_size(&path).unwrap_or(0)
            } else {
                0
            }
        } else {
            md.len()
        };

        entries.push((path, size, md.is_dir()));
    }

    match cli.sort {
        SortOrder::NameAsc => {
            entries.sort_by(|a, b| a.0.file_name().cmp(&b.0.file_name()));
        }
        SortOrder::NameDesc => {
            entries.sort_by(|a, b| b.0.file_name().cmp(&a.0.file_name()));
        }
        SortOrder::SizeAsc => {
            entries.sort_by_key(|a| a.1);
        }
        SortOrder::SizeDesc => {
            entries.sort_by_key(|b| std::cmp::Reverse(b.1));
        }
        SortOrder::None => {}
    }

    for (path, size, is_dir) in entries {
        if is_dir {
            if cli.folder_size {
                let folder_size_mb = size as f64 / 1024.0 / 1024.0;
                if cli.nocolor {
                    println!("•  📁 {}       {:.2} MB", path.display(), folder_size_mb);
                } else {
                    println!("•  📁 {}       {:.2} MB", path.display().to_string().blue().bold(), folder_size_mb);
                }
            } else {
                if cli.nocolor {
                    println!("•  📁 {}       N/A", path.display());
                } else {
                    println!("•  📁 {}       N/A", path.display().to_string().blue().bold());
                }
            }
        } else {
            let sizekb = size as f64 / 1024.0;
            if cli.nocolor {
                println!("•  📄 {}       {:.2} KB", path.display(), sizekb);
            } else {
                println!("•  📄 {}       {:.2} KB", path.display().to_string().green(), sizekb);
            }
        }
    }
}
