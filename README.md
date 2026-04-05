# shuz 📝

A simple, lightweight, and fast terminal-based text editor written entirely in Rust. 

Whether you need to quickly tweak a configuration file, write some code, or take quick notes without leaving your terminal, `shuz` provides a clean and distraction-free editing experience straight from your command line. 

## ✨ Features
* **Terminal-Native:** Runs entirely within your terminal, making it perfect for SSH sessions or quick edits.
* **Lightweight & Fast:** Built with Rust, meaning zero bloat, minimal memory usage, and instant startup times.
* **Minimalist Interface:** Designed to be intuitive and simple, so you can focus entirely on your text without navigating complex menus.
* **Cross-Platform:** Can be compiled and run anywhere Rust runs.

## 🛠️ Tech Stack
* **Language:** [Rust](https://www.rust-lang.org/)
* Compiled natively via `cargo`.

## 📦 Getting Started

### Prerequisites
Make sure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install) on your machine.

### Installation & Build
1. **Clone the repository:**
   ```bash
   git clone [https://github.com/Pandakil1er/shuz.git](https://github.com/Pandakil1er/shuz.git)
   cd shuz
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the editor:**
   ```bash
   cargo run 
   # OR run the compiled binary directly:
   ./target/release/shuz
   ```

*(Optional)* You can also pass a file directly to open it:
```bash
cargo run -- filename.txt
```

## 🤝 Contributing
Contributions, issues, and feature requests are welcome! If you want to make `shuz` even better, feel free to fork the repository, make your changes, and submit a Pull Request.

## 📄 License
This project is open-source. Feel free to use, modify, and distribute it as you see fit!
