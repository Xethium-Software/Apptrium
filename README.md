```markdown
# Linux Package Manager with Beginner-Friendly GUI 🌟

Welcome to our **Linux package manager**! This tool is designed to make managing packages easier for everyone, featuring a simple GUI. It currently supports **Arch Linux** and **Debian-based distros** with plans for expansion.

---

## 🚀 Features
- **Beginner-friendly GUI** to simplify package management tasks.
- **Multi-distro support**, starting with Arch Linux and Debian-based distributions.
- Built with **Rust** for speed and reliability.

---

## 🛠️ Installation Guide

### Prerequisites
Before building and running the project, ensure Rust is installed on your system.

#### For Arch Linux:
1. Install Rustup:
   ```bash
   sudo pacman -S rustup
   ```
2. Set the default Rust version to stable:
   ```bash
   rustup default stable
   ```

#### For Debian-based Distros (e.g., Ubuntu):
1. Update your package list and install Rustup:
   ```bash
   sudo apt update && sudo apt install rustup
   ```
2. Set the default Rust version to stable:
   ```bash
   rustup default stable
   ```

---

## 🛠️ How to Build the Project

1. Clone the repository:
   ```bash
   git clone <repo_url>
   cd <project_directory>
   ```

2. Build the project using `cargo`:
   ```bash
   cargo build
   ```
   This will compile the project and fetch all required dependencies.

3. Run the project:
   ```bash
   cargo run
   ```

---

## ⬇️ Getting the Latest Release
If you prefer not to build the project yourself, download the **latest release** from the [Releases](https://github.com/<repo_url>/releases) tab.  
Simply download the binary for your system and run it.

> **Note:** If there are no releases yet, the project is still under development. Stay tuned for updates!

---

## 📦 Dependencies
The project uses the following Rust libraries:

| Library           | Version   | Description                            |
|--------------------|-----------|----------------------------------------|
| `dirs`            | 5.0.1     | Convenient platform-specific paths.    |
| `env_logger`      | 0.11.5    | Logging setup made simple.             |
| `gtk`             | 0.6.1     | GTK4 for GUI (with `v4_6` feature).    |
| `log`             | 0.4.22    | Structured logging for Rust apps.      |
| `reqwest`         | 0.12.9    | HTTP requests (`blocking` feature).    |
| `serde`           | 1.0.215   | Serialization and deserialization.     |
| `serde_json`      | 1.0.133   | JSON parsing and manipulation.         |

---

## 💻 Contributors
This project is brought to you by:
- **[Barkotbb](https://github.com/barkotbb)**  
- **[Theridev (Theri)](https://github.com/theridev)**  


---

Enjoy managing your Linux packages the easy way! 🎉
```
