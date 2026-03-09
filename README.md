# Command Line Chemestry Companion (CLCC)
## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [References](#references)


## Installation

### Downloading from Release
Download the binary for your operating system from [here](https://github.com/LennikGamez/CommandLineChemstryCompanion/releases/latest)
You can rename the file to make it easier to work with

#### On Linux
You can move the binary into a directory like ```/usr/local/bin``` which is in your ```$PATH``` variable
```bash
mv path/to/downloaded/file /usr/local/bin/clcc
```
This command moves the downloaded file into the /usr/local/bin directory and renames the file to clcc, so that you can execute the script in the terminal with ```clcc```

## If you have problems  executing the file on Linux run
```bash
chmod +x <filename>
```

### Compiling with cargo
Download the source code and run
```bash
cargo build --release
```
## Usage

### Basic Example

Calculate the atomic mass of Water (H_2O): 
```bash
Bitte gib die Strukturformel an:
> H_2O
```


## References
- Data of the periodic table [here](https://pse-info.de/de/data) 

