# RustLearner 🚀  

**RustLearner** is an interactive learning platform designed to teach the Rust programming language through guided lessons and hands-on exercises. Whether you're a beginner or an experienced programmer, RustLearner provides a structured approach to mastering Rust's powerful features.  

## Features  

-  **Interactive CLI** – Learn Rust step by step with an engaging command-line interface.  
-  **Web Interface** – Access lessons and exercises via a built-in web server.  
-  **Comprehensive Lessons** – Covers Rust fundamentals, advanced topics, and practical exercises.  
-  **Hands-On Exercises** – Test your knowledge with interactive coding challenges.  
-  **Progress Tracking** – Keep track of completed lessons and measure your learning progress.  

## 🛠 Installation & Setup  

### Prerequisites  
- Ensure you have [Rust & Cargo](https://www.rust-lang.org/tools/install) installed.  
- Clone the repository:  

  ```sh
  git clone https://github.com/thanksrest/learnrust.git
  cd rust_learner
  ```  

- Build the project:  

  ```sh
  cargo build
  ```  

##  Usage  

### Start Interactive CLI  
```sh
cargo run -- interactive
```  
The CLI provides a guided, step-by-step Rust learning experience. Navigate through lessons and complete exercises directly in your terminal.  

### Start Web Server  
```sh
cargo run -- web
```  
This launches a local web server where you can access Rust lessons and exercises from your browser. By default, it runs on `http://127.0.0.1:8080`.  

##  Lesson Topics  

| Category | Lesson Name |
|----------|------------|
| **Basics** | Variables & Types, Control Flow |
| **Advanced** | Ownership, Traits |

More lessons will be added in future updates! 🚀  

## 🤝 Contributing  

contributions are welcome! If you have ideas for new lessons, improvements, or bug fixes:  
1. Fork the repository  
2. Create a new branch (`git checkout -b feature-branch`)  
3. Commit your changes (`git commit -m "Add new lesson"`)  
4. Push to the branch (`git push origin feature-branch`)  
5. Open a Pull Request  

## 📜 License  

RustLearner is open-source and licensed under the [MIT License](LICENSE).  

