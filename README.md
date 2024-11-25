This is a Linux package manager with a beginner-friendly GUI, written in Rust, to simplify package management. It supports multiple Linux distributions and will initially be released for Arch Linux and Debian-based distros.

---

**How to build and run?**  
First, make sure Rust is installed. Here’s how:

1. **For Arch Linux:**  
   Run the following commands:  
   ```
   sudo pacman -S rustup
   rustup default stable
   ```

2. **For Debian-based distros (e.g., Ubuntu):**  
   Run the following commands:  
   ```
   sudo apt update && sudo apt install rustup
   rustup default stable
   ```

**Building the project:**  
1. Clone the repository:  
   ```
   git clone https://github.com/Xethium-Software/Apptrium/
   cd Apptrium
   ```

2. Build the project using Cargo:  
   ```
   cargo build
   ```
   This will compile all the libraries and dependencies.

3. Run the project:  
   ```
   cargo run
   ```

Alternatively, download the latest release from the Releases tab and run the pre-built binary. If no release is available, it means the project is still under development.

---

**Dependencies:**  
This project uses the following Rust libraries:  
- dirs = "5.0.1"  
- env_logger = "0.11.5"  
- gtk = { version = "0.6.1", package = "gtk4", features = ["v4_6"] }  
- log = "0.4.22"  
- reqwest = { version = "0.12.9", features = ["blocking"] }  
- serde = { version = "1.0.215", features = ["derive"] }  
- serde_json = "1.0.133"  

---

**Contributors:**  
- ShaderHex: https://github.com/ShaderHex  
- Theridev (Theri): https://github.com/theridev
