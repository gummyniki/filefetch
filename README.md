# 📦 filefetch

**A simple Rust CLI tool to display information about the current folder.**  
Shows the total size, number of files and folders, and prints each entry with a neat emoji.

---

## 🚀 Features

- 📁 Shows current directory path
- 📦 Displays folder size in MB
- 🔢 Counts files and optionally subdirectories
- 📄 Lists files and folders with colorful output
- 🎛️ Optional flags for no-color, subdirectory counting, and sorting
- 🔃 Sort entries by name or size (ascending or descending)

---

## 🛠️ Installation

From [crates.io](https://crates.io/crates/filefetch):


```bash
cargo install filefetch
```

### for Arch Linux

From [AUR](https://aur.archlinux.org/packages/filefetch):

```bash
yay -S filefetch
```

*(btw if you're installing from crates.io make sure to have cargo installed, it's the package manager for rust)*


## 🧪 Usage
```bash
filefetch
```

# Optional flags:

    --nocolor – disable colored output

    --recursive – include files and folders inside subdirectories

    --folder-size - shows folder sizes *(will take longer)*

    --sort - sort entries by name or size *(none, name-asc, name-desc, size-asc, size-desc)*


# 📂 Example Output


```bash
📁 Current Directory: /home/user/projects
📦 Folder Size: 13.42 MB
📦 Number of entries: 8
📄 Files:
•  📁 src
•  📄 Cargo.toml
•  📄 README.md

```
