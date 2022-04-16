# Getting Started

This guide will help you set up a development environment to work with NtHiM as well as other projects built with Rust.

* Set up Rust
* Development Environment Setup
* Get NtHiM (and, start contributing!)


## Are you new to Rust!?

* [Learn Rust](https://www.rust-lang.org/learn)
* [Read the Book](https://doc.rust-lang.org/book/)
* [Do the Rustlings course!](https://github.com/rust-lang/rustlings/)
* [Check out Rust by Example!](https://doc.rust-lang.org/stable/rust-by-example/)


## Set up Rust

You will need the latest version (or at least a newer release) of Rust and Cargo to perform the manual build for NtHiM, to ensure that the build process proceeds properly.

It is highly recommended to use Rust and Cargo using **rustup**. If you would like to install it that way, you can go through the instructions provided at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) or [https://rustup.rs/](https://rustup.rs/).


## Development Environment Setup

You will need to have a proper IDE to work with any of the Rust projects including NtHiM. This doesn't mean that you can't use a text editor; a simple text editor is completely enough to get you started too.

If you will be using Visual Studio Code, I would recommend you to get one of the following extensions:
* [Rust](https://github.com/rust-lang/vscode-rust) (The Rust Programming Language),
* [rust-analyzer](https://github.com/rust-lang/rust-analyzer) (matklad)

**rust-analyzer** is also available for Emacs, Vim/NeoVim, Sublime Text, and many other text editors. You can read its manual [here](https://rust-analyzer.github.io/manual.html).


## Get NtHiM (and, start contributing!)

1. Clone the repository, `git clone https://github.com/TheBinitGhimire/NtHiM` (or you may clone a forked one);
2. Open up the NtHiM local repository directory in your preferred text editor or IDE,
3. Start making changes as per your wish,
4. Use the `cargo build` command to build a debug binary,
5. Go inside the newly-created **target** folder, and open the **debug** folder inside it, `cd target/debug`;
6. You will find **NtHiM.exe** (on Microsoft Windows) or **NtHiM** binary (on most of the other platforms), which you can execute to test your changes.

If you would like to get a release binary for **NtHiM**, you can use the **`--release`** flag in **Step 4**, and go to the **release** folder (i.e. `cd target/release`) instead in **Step 5**.


## Wishing to contribute for others to see what you build?

Please go through the [Contributing Guide](https://github.com/TheBinitGhimire/NtHiM/blob/main/.github/CONTRIBUTING.md)!

