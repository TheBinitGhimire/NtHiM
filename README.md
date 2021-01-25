<h1 align="center" style="font-size:36px;font-weight:bold;">
        NtHiM
</h1>
<h4 align="center">
    <strong>Powered by BINIT GHIMIRE (<a href='https://twitter.com/WHOISbinit' target="_blank">@WHOISbinit</a>)</strong>
</h4>
<p align="center">
    <img src="src/screenshot.png">
</p>
<h3 align="center"><strong>N<strong>ow, <strong>t</strong>he <strong>H</strong>ost <strong>i</strong>s <strong>M</strong>ine!</h3><hr/>
<h2 align="center">Super Fast Sub-domain Takeover Detection!</h2>
<p align="center">
    <a href="https://www.rust-lang.org/" target="_blank"><img src="https://ForTheBadge.com/images/badges/made-with-rust.svg"></a>
</p>

## Installation
### Method 1: Using Pre-compiled Binaries
The pre-compiled binaries for different systems are available in the [**Releases**](https://github.com/TheBinitGhimire/NtHiM/releases) page. You can download the one suitable for your system, unzip the file and start using NtHiM.

### Method 2: Manual Build
You will need Cargo to perform the manual build for NtHiM.
If you have Cargo installed, you can simply follow the steps below:
1. Clone this repository, `git clone https://github.com/TheBinitGhimire/NtHiM`;
2. Go inside the folder, `cd NtHiM`;
3. Use the `cargo build` command,
4. Go inside the newly-created **target** folder, and open the **debug** folder inside it, `cd target/debug`;
5. You will find **NtHiM.exe** (on Microsoft Windows) or **NtHiM** binary (on Linux).

## Usage
### Use Case 1 (Single Target):

```bash
NtHiM -t https://example.example.com
```

### Use Case 2 (Multiple Targets):

```bash
NtHiM -f hostnames.txt
```

## Examples
### Single Target
![Single Target](src/example1.png)

### Multiple Targets using Concurrent Threads
![Multiple Targets using Concurrent Threads](src/example2.png)
