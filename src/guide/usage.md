# Usage

The usage flags for NtHiM are provided below.

| Flag | Description                           | Example                              |
| ---- | ------------------------------------- | ------------------------------------ |
| -h   | Display help related to usage!        | NtHiM -h                             |
| -t   | Scan a single target!                 | NtHiM -t https://example.example.com |
| -f   | Scan a list of targets from a file!   | NtHiM -f hostnames.txt               |
| -c   | Number of Concurrent Threads!         | NtHiM -c 100 -f hostnames.txt        |
| -s   | Timeout for connections (in seconds)! | NtHiM -s 4 -f hostnames.txt          |
| -v   | Enable Verbose Mode!                  | NtHiM -v -f hostnames.txt            |
| -o   | Write output to file!                 | NtHiM -f hostnames.txt -o output.txt |
| -u   | Update signature cache!               | NtHiM -u                             |
| -V   | Display the version information!      | NtHiM -V                             |

*By default, **NtHiM** uses **`10`** **concurrent threads**, and **`5`** seconds of **timeout for connections**.*

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

![NtHiM Usage Demonstration](https://raw.githubusercontent.com/TheBinitGhimire/NtHiM/main/images/demonstration.gif)

***