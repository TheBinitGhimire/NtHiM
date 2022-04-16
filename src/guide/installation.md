# Installation

### Method 1: Using Pre-compiled Binaries
The pre-compiled binaries for different systems are available in the [**Releases**](https://github.com/TheBinitGhimire/NtHiM/releases) page. You can download the one suitable for your system, unzip the file and start using NtHiM.

### Method 2: Using Crates.io
**NtHiM** is available on **[Crates.io](https://crates.io/crates/NtHiM)**. So, if you have Rust installed on your system, you can simply install **NtHiM** with the following command:

```bash
cargo install NtHiM
```

### Method 3: Manual Build
You will need the latest version (or at least a newer release) of Cargo to perform the manual build for NtHiM, to ensure that the build process proceeds properly.

It is highly recommended to use Rust and Cargo using **rustup**. If you would like to install it that way, you can go through the instructions provided at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) or [https://rustup.rs/](https://rustup.rs/).

If you have Cargo installed, you can simply follow the steps below:
1. Clone this repository, `git clone https://github.com/TheBinitGhimire/NtHiM`;
2. Go inside the folder, `cd NtHiM`;
3. Use the `cargo build` command,
4. Go inside the newly-created **target** folder, and open the **debug** folder inside it, `cd target/debug`;
5. You will find **NtHiM.exe** (on Microsoft Windows) or **NtHiM** binary (on most of the other platforms).

If you would like to get a release binary for **NtHiM**, you can use the **`--release`** flag in **Step 3**, and go to the **release** folder (i.e. `cd target/release`) instead in **Step 4**.

**For older versions of NtHiM as well as some infrequently used or older platforms or system architectures:**
Before doing a manual build of NtHiM or installing through **Crates.io** with **`cargo`**, you might need to have OpenSSL installed in your system. Sometimes if you get an error during the manual building process, then the odds of the error taking place due to OpenSSL are high. Please check out the [issue #1](https://github.com/TheBinitGhimire/NtHiM/issues/1) to figure out a solution for this issue.

The installation walkthrough for **NtHiM** has been uploaded to YouTube, covering all of these three methods, and you can watch the video here: **[How to Install and Use NtHiM (Now, the Host is Mine!)? Super Fast Sub-domain Takeover Detection!](https://youtu.be/CUTbqFhRjwY)**