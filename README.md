# archeal
Archeal is a website archiving tool written in Rust. It aims to archive the content of web pages, capturing HTML structures. 
Please note that the project is currently a work in progress (WIP).

## Why?
Most knowledge in Internet is in a real danger of being due to shutdowns, DDoS attacks etc. Due to this many people use websites like archive.org and archive.md.
The Problem is most of these sites are centrilased and complex to selfhost. Archeal uses a much simpler approach for archiving. infact you dont even have to contuniosly host it.



## Dependencies
- Rust Toolchain. MSRV is 1.73
- OpenSSL**

** Only on UNIX-Like systems.

## Installation
1. Clone the repository
```
git clone https://github.com/AlexanderMaxRanabel/archeal.git
```
2. Change to Archeal directory
```
cd archeal
```
3. Compile the project
```
cargo build --release
```

## Usage
{} means its not an actual part of the command syntax.

```
./archeal --url https://archive.md --depth {True or false}
```

## Licensing
Archeal is licensed under GNU Affero General Public License 3.0. See LICENSE for more information.
