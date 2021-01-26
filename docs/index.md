<h1 align="center" style="font-size:36px;font-weight:bold;">
        NtHiM
</h1>
<h4 align="center">
    <strong>Powered by BINIT GHIMIRE (<a href='https://twitter.com/WHOISbinit' target="_blank">@WHOISbinit</a>)</strong>
</h4>
<p align="center">
    <img src="src/screenshot.png" />
</p>
<h3 align="center"><strong>N<strong>ow, <strong>t</strong>he <strong>H</strong>ost <strong>i</strong>s <strong>M</strong>ine!</h3><hr/>
<h2 align="center">Super Fast Sub-domain Takeover Detection!</h2>
<p align="center">
    <a href="https://www.rust-lang.org/" target="_blank"><img src="https://ForTheBadge.com/images/badges/made-with-rust.svg" /></a>
</p>


***

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

***

## Usage

| Flag  | Description                         | Example                                  |
| ----- | ----------------------------------- | ---------------------------------------- |
|  -h   | Display help related to usage!      | NtHiM -h                                 |
|  -t   | Scan a single target!               | NtHiM -t https://example.example.com     |
|  -f   | Scan a list of targets from a file! | NtHiM -f hostnames.txt                   |
|  -c   | Number of Concurrent Threads!       | NtHiM -c 100 -f hostnames.txt            |
|   -V  | Display the version information!    | NtHiM -V                                 |

***

### Use Case 1 (Single Target):

```bash
NtHiM -t https://example.example.com
```

### Use Case 2 (Multiple Targets):

```bash
NtHiM -f hostnames.txt
```

***

### Usage Demonstration:

![NtHiM Usage Demonstration](src/demonstration.gif)

***

## Examples

### Single Target
![Single Target](src/example1.png)

### Multiple Targets using Concurrent Threads
![Multiple Targets using Concurrent Threads](src/example2.png)

***

## Workflow

### Platform Identification
**NtHiM** uses the data provided in **[EdOverflow/can-i-take-over-xyz](https://github.com/EdOverflow/can-i-take-over-xyz)** for the platform identification.

***

## Contributions and Feature Requests
<p align="center">
    <a href="https://github.com/TheBinitGhimire/NtHiM/pulls"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square" /></a>
</p>


If you are interested in contributing in the development of <strong>NtHiM</strong>, you can feel free to create a <strong>Pull Request</strong> with modifications in the original code, or you shall open up a new <strong>issue</strong>, and I will try to include the feature as requested.

There is no restriction on anyone for contributing to the development of <strong>NtHiM</strong>. If you would like to contribute, you can feel free to do so.

