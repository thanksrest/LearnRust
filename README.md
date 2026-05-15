# LearnRust 🚀  

Welcome! **LearnRust** is an interactive learning platform that teaches Rust through guided lessons and hands-on exercises. Whether you're just starting out or already have programming experience, this project walks you through Rust's most powerful concepts at your own pace.  

> **Quick tip:** The `main` branch contains the stable, working version. Other branches are for experimental features – head there if you want to contribute or dig into the code!

## Table of Contents
- [Features](#features)
- [Installation & Setup](#-installation--setup)
- [Usage](#-usage)
- [Lesson Topics](#-lesson-topics)
- [Contributing](#-contributing)
- [License](#-license)

## Features  

- **Interactive CLI** – Learn Rust step by step right from your terminal. It's fast, responsive, and perfect for focused learning.
- **Web Interface** – Prefer a browser? Launch a local web server and explore lessons from there. No installation hassles.
- **Comprehensive Lessons** – We cover everything from Rust basics (variables, ownership, borrowing) to more advanced topics like traits and async programming.
- **Hands-On Exercises** – Theory is great, but learning Rust means writing Rust. Each lesson includes interactive coding challenges.
- **Progress Tracking** – Keep tabs on what you've completed and see how far you've come.  

## 🛠 Installation & Setup  

### Prerequisites  
First, make sure you have [Rust & Cargo installed](https://www.rust-lang.org/tools/install). If you're new to Rust, that link will guide you through the setup.

### Get Started  
Clone the repo and build:

```sh
git clone https://github.com/thanksrest/learnrust.git
cd learnrust
cargo build
```  

##  Usage  

### Terminal Mode (Interactive CLI)  
```sh
cargo run -- interactive
```  
This launches an interactive tutorial right in your terminal. You'll navigate through lessons, see explanations, and write code – all without leaving your shell.

### Browser Mode (Web Interface)  
```sh
cargo run -- web
```  
Prefer a visual experience? This spins up a local web server at `http://localhost:8080` where you can explore lessons in your browser.  

##  Lesson Topics  

Currently covering:

| Category | Topics |
|----------|--------|
| **Basics** | Variables & Types, Control Flow, Functions |
| **Intermediate** | Ownership & Borrowing, Pattern Matching |
| **Advanced** | Traits, Error Handling |

We're always adding more! Have an idea for a lesson? [Open an issue](https://github.com/thanksrest/learnrust/issues) or submit a PR.  

## 🤝 Contributing  

We'd love to have you contribute! Whether it's a new lesson, a bug fix, or documentation improvements, here's how to get involved:

1. Fork the repository  
2. Create a feature branch (`git checkout -b feature/your-idea`)  
3. Make your changes and test them  
4. Commit with a clear message (`git commit -m "Add lesson on iterators"`)  
5. Push to your fork (`git push origin feature/your-idea`)  
6. Open a Pull Request and describe what you changed

First time contributing? Don't worry – we're friendly and happy to help!  

## 📜 License  

LearnRust is open-source and licensed under the [MIT License](LICENSE).  
