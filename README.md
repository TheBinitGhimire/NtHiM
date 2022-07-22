<!-- NtH1M | Now, the Host is Mine! - Super Fast Sub-domain Takeover Detection! -->

<p align="center">
        <a href="https://github.com/c4p-n1ck/NtH1M/stargazers" target="_blank"><img alt="GitHub Stars" src="https://img.shields.io/github/stars/c4p-n1ck/NtH1M?style=for-the-badge" /></a>
        &nbsp;&nbsp;&nbsp;
        <a href="https://github.com/c4p-n1ck/NtH1M/network/members" target="_blank"><img alt="GitHub Forks" src="https://img.shields.io/github/forks/c4p-n1ck/NtH1M?style=for-the-badge" /></a>
        &nbsp;&nbsp;&nbsp;
        <a href="https://github.com/c4p-n1ck/NtH1M/issues" target="_blank"><img alt="GitHub Issues" src="https://img.shields.io/github/issues/c4p-n1ck/NtH1M?style=for-the-badge" /></a>
        <br />
        <img src="images/header.png" alt="Now, the Host is Mine! - Super Fast Sub-domain Takeover Detection" title="NtH1M | Now, the Host is Mine!">
        <br />
        &nbsp;&nbsp;&nbsp;
        <a href="https://github.com/c4p-n1ck/NtH1M/releases" target="_blank"><img alt="GitHub Downloads" src="https://img.shields.io/github/downloads/c4p-n1ck/NtH1M/total.svg?style=for-the-badge&label=GitHub+Downloads" /></a>
        &nbsp;&nbsp;&nbsp;
        <a href="https://crates.io/crates/NtH1M" target="_blank"><img alt="Cargo Installs" src="https://img.shields.io/crates/d/NtH1M?style=for-the-badge&label=Cargo+Installs" /></a>
</p>


***


<h1 align="center"> NtH1M - Super Fast Sub-domain Takeover Detection </h1>
<p align="center">
    <a href="https://github.com/c4p-n1ck/NtH1M/releases/tag/0.1.4" target="_blank"><img src="https://img.shields.io/badge/latest-0.1.4-blue?style=for-the-badge&label=Latest+Release" height="36" /></a>
    &nbsp;&nbsp;
    <a href="https://www.rust-lang.org/" target="_blank"><img src="https://forthebadge.com/images/badges/made-with-rust.svg" height="36" /></a>
    &nbsp;&nbsp;
    <a href="https://whoisbinit.me/NtHiM/" target="_blank"><img src="https://img.shields.io/badge/documentation-blue?style=for-the-badge&label=Read&logo=docsdotrs&labelColor=c21919&color=bd1799" height="36" /></a>
</p>


# Notice

> This is a sad notice that our Internet Hero (@TheBinitGhimire) had recent demise on 26th of June, 2022. Since has has passed away, and I was the one who implemented online repo system in collaboration with [Penterter-Nepal](https://github.com/Pentester-Nepal) using [this](https://git.io/signatures_json) as repo and there was an issue for using local/custom `signatures.json` file and not from the repo. Hence, I've implemented it with a major release version (everying is almost same except for naming since we need to make releases) and few misc changes.


***


## Read the full [**NtH1M Documentation**](https://whoisbinit.me/NtHiM/)!

This guide consists of documentations for users and developers using or contributing to NtH1M.

***

## Installation

### Method 1: Using Pre-compiled Binaries
The pre-compiled binaries for different systems are available in the [**Releases**](https://github.com/c4p-n1ck/NtH1M/releases) page. You can download the one suitable for your system, unzip the file and start using NtH1M.

### Method 2: Using Crates.io
**NtH1M** is available on **[Crates.io](https://crates.io/crates/NtH1M)**. So, if you have Rust installed on your system, you can simply install **NtH1M** with the following command:

```bash
cargo install NtH1M
```

### Method 3: Manual Build
You will need the latest version (or at least a newer release) of Cargo to perform the manual build for NtH1M, to ensure that the build process proceeds properly.

It is highly recommended to use Rust and Cargo using **rustup**. If you would like to install it that way, you can go through the instructions provided at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) or [https://rustup.rs/](https://rustup.rs/).

If you have Cargo installed, you can simply follow the steps below:
1. Clone this repository, `git clone https://github.com/c4p-n1ck/NtH1M`;
2. Go inside the folder, `cd NtH1M`;
3. Use the `cargo build` command,
4. Go inside the newly-created **target** folder, and open the **debug** folder inside it, `cd target/debug`;
5. You will find **NtH1M.exe** (on Microsoft Windows) or **NtH1M** binary (on most of the other platforms).

If you would like to get a release binary for **NtH1M**, you can use the **`--release`** flag in **Step 3**, and go to the **release** folder (i.e. `cd target/release`) instead in **Step 4**.

**For older versions of NtH1M as well as some infrequently used or older platforms or system architectures:**
Before doing a manual build of NtH1M or installing through **Crates.io** with **`cargo`**, you might need to have OpenSSL installed in your system. Sometimes if you get an error during the manual building process, then the odds of the error taking place due to OpenSSL are high. Please check out the [issue #1](https://github.com/c4p-n1ck/NtH1M/issues/1) to figure out a solution for this issue.

The installation walkthrough for **NtH1M** has been uploaded to YouTube, covering all of these three methods, and you can watch the video here: **[How to Install and Use NtH1M (Now, the Host is Mine!)? Super Fast Sub-domain Takeover Detection!](https://youtu.be/CUTbqFhRjwY)**


***

## Usage

| Flag | Description                           | Example                              |
| ---- | ------------------------------------- | ------------------------------------ |
| -h   | Display help related to usage!        | NtH1M -h                             |
| -t   | Scan a single target.                 | NtH1M -t https://example.example.com |
| -f   | Scan a list of targets from a file.   | NtH1M -f hostnames.txt               |
| -c   | Number of Concurrent Threads.         | NtH1M -c 100 -f hostnames.txt        |
| -s   | Timeout for connections (in seconds)  | NtH1M -s 4 -f hostnames.txt          |
| -v   | Enable Verbose Mode.                  | NtH1M -v -f hostnames.txt            |
| -o   | Write output to file.                 | NtH1M -f hostnames.txt -o output.txt |
| -p   | Path to custom signatures.json file.  | NtH1M -p signatures.json -f host.txt |
| -u   | Update signature cache.               | NtH1M -u                             |
| -V   | Display the version information.      | NtH1M -V                             |

*By default, **NtH1M** uses **`10`** **concurrent threads**, and **`5`** seconds of **timeout for connections**.*

***

### Use Case 1 (Single Target):

```bash
NtH1M -t https://example.example.com
```

### Use Case 2 (Multiple Targets):

```bash
NtH1M -f hostnames.txt
```

***

### Usage Demonstration (without -p flag):

![NtH1M Usage Demonstration (without -p flag)](images/demonstration.gif)

***

## Examples

### Single Target
![Single Target](images/examples/example1.png)

### Multiple Targets using Concurrent Threads
![Multiple Targets using Concurrent Threads](images/examples/example2.png)

***

## Workflow

### Platform Identification
**NtH1M** uses the data provided in **[EdOverflow/can-i-take-over-xyz](https://github.com/EdOverflow/can-i-take-over-xyz)** for the platform identification.

***

## Frequently Asked Questions (FAQs)
If you have any questions regarding **NtH1M**, please raise an issue by going to the **[Issues](https://github.com/c4p-n1ck/NtH1M/issues)** page.

Some of your queries might have been answered in one of the existing issues, so please make sure to check the Issues with the **[FAQ](https://github.com/c4p-n1ck/NtH1M/issues?q=is%3Aissue+label%3AFAQ)** label before raising an issue on your own.

***

## Contributions and Feature Requests
<p align="center">
    <a href="https://github.com/c4p-n1ck/NtH1M/pulls"><img src="https://img.shields.io/badge/Pull%20Requests-welcome-brightgreen.svg?style=for-the-badge" /></a>
</p>


We welcome contributions from **NtH1M** users, developers and anyone who is interested to help in upgrading the status of the project. Therefore, we have prepared a [Contributing Guide](.github/CONTRIBUTING.md) that would be helpful for future contributors.

If you are interested in contributing in the development of **NtH1M**, you can feel free to create a **Pull Request** with modifications in the original code, or you shall open up a new **issue**, and we will try to include the feature as requested.

There is no restriction on anyone for contributing to the development of **NtH1M**. If you would like to contribute, you can feel free to do so, but please make sure to go through out [Contributing Guide](.github/CONTRIBUTING.md) before creating a Pull Request.

***

## Code of Conduct

Our Code of Conduct is available at [Contributor Covenant Code of Conduct](.github/CODE_OF_CONDUCT.md).

***

## License

[**NtH1M**](https://github.com/c4p-n1ck/NtH1M) is licensed under the [MIT License](https://github.com/c4p-n1ck/NtH1M/blob/main/LICENSE).
