# parcch


🚀 Vetinary Mgt System
This is a Rust-based web application built with Actix Web, designed to manage veterinary information efficiently.

It uses cargo-watch to enable automatic recompilation and live reload during development.

📦 Prerequisites
Before running this project, ensure you have the following installed:

Rust (rustup)

cargo-watch (for live reload during development)

To install Rust:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
To verify Rust installation:

rustc --version
cargo --version
⚙️ Installation
Clone the repository:

git clone https://github.com/Akuafo-Patawah12/Vetinary-Mgt-System.git
cd Vetinary-Mgt-System
Install cargo-watch (if not already installed):

cargo install cargo-watch
Build the project:

cargo build
🚀 Running the Project
To run your project with cargo-watch so it automatically restarts on file changes:


cargo watch -c -w src -x run
This watches your src directory and re-runs your project when you save changes.


🧪 Extra Commands
Watch and check for errors:

cargo watch -x check
Watch and run tests:

cargo watch -x test
