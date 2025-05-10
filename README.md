![build](https://github.com/srdlj/dupels/actions/workflows/rust.yml/badge.svg) [![Coverage Status](https://coveralls.io/repos/github/srdlj/dupels/badge.svg?branch=main)](https://coveralls.io/github/srdlj/dupels?branch=main) ![GitHub Release](https://img.shields.io/github/v/release/srdlj/dupels)
# dupels

Inspired by the `ls` command but lists directory contents grouped by their checksum (MD5, options for other cryptographic hash functions coming soon). The main use case of this tool is to identify duplicate files nested directories efficiently. duplicates efficiently.

## Example

The following example shows all audio samples in a drumkit which are duplicates.
The `-d` option specifies to go at most 2 subdirectories deep to find files and the `-o` option omits all files with
a unique checksum.

The `>--` seperator is a boundary for each checksum group (in this case groups of duplicates as we've omited all unique files).

```bash
$ ./target/debug/dupels -d 2 -o ~/drum_kits/B-Wheezy 
/user/home/drum_kits/B-Wheezy/Vocals/Yeah2.wav
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/yeah_2.wav
>--
/user/home/drum_kits/B-Wheezy/Vocals/hey6.wav
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/A!.wav
>--
/user/home/drum_kits/B-Wheezy/Vocals/BWVox.wav
/user/home/drum_kits/B-Wheezy/Vocals/BStack.wav
>--
/user/home/drum_kits/B-Wheezy/LiveDrums/ConcertBD1_3.wav
/user/home/drum_kits/B-Wheezy/LiveDrums/ConcertBD1_2.wav
>--
/user/home/drum_kits/B-Wheezy/Vocals/Yup.wav
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/yup.wav
>--
/user/home/drum_kits/B-Wheezy/Vocals/hey3.wav.asd
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/hey32_2.wav.asd
>--
/user/home/drum_kits/B-Wheezy/Vocals/hey6.wav.asd
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/A!.wav.asd
>--
/user/home/drum_kits/B-Wheezy/Vocals/SReddVox.wav
/user/home/drum_kits/B-Wheezy/Vocals/ShawtyRedd_3.wav
>--
/user/home/drum_kits/B-Wheezy/Vocals/Yeah2.wav.asd
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/yeah_2.wav.asd
>--
/user/home/drum_kits/B-Wheezy/Vocals/hey10.wav.asd
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/aye_2.wav.asd
>--
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/Friday132.wav.asd
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/Friday13.wav.asd
>--
```

## Installation

### Linux

You can download the latest release from the [GitHub Releases page](https://github.com/srdlj/dupels/releases) and extract the appropriate archive for your system:

```bash
wget https://github.com/srdlj/dupels/releases/latest/download/dupels-linux-<version>.tar.gz
tar -xzf dupels-linux-<version>.tar.gz
cd dupels-linux-<version>
./dupels-cli --help
```

Or, for the zip archive:

```bash
wget https://github.com/srdlj/dupels/releases/latest/download/dupels-linux-<version>.zip
unzip dupels-linux-<version>.zip
cd dupels-linux-<version>
./dupels-cli --help
```

### macOS

You can download the latest release from the [GitHub Releases page](https://github.com/srdlj/dupels/releases) and extract the appropriate archive for your system:

```bash
curl -LO https://github.com/srdlj/dupels/releases/latest/download/dupels-macos-<version>.tar.gz
tar -xzf dupels-macos-<version>.tar.gz
cd dupels-macos-<version>
./dupels-cli --help
```

Or, for the zip archive:

```bash
curl -LO https://github.com/srdlj/dupels/releases/latest/download/dupels-macos-<version>.zip
unzip dupels-macos-<version>.zip
cd dupels-macos-<version>
./dupels-cli --help
```

### Windows

Download the Windows release from the [GitHub Releases page](https://github.com/srdlj/dupels/releases):

With zip:

```PowerShell
Expand-Archive -Path .\dupels-windows-<version>.zip -DestinationPath .\dupels-windows-<version>
cd .\dupels-windows-<version>
.\dupels-cli.exe --help
```

Or, with tar:

```
tar -xzf .\dupels-windows-<version>.tar.gz
cd .\dupels-windows-<version>
.\dupels-cli.exe --help
```

### Building from Source

Ensure you have [Russt](https://rustup.rs/) installed, then run:

```bash
git clone https://github.com/srdlj/dupels.git
cd dupels
cargo build --release --package dupels-cli
```

The compiled binary will be located at:

- `target/release/dupels-cli (Linux/macOS)`
- `target/release/dupels-cli.exe (Windows)`

You can then run:

`./target/release/dupels-cli --help`

or on Windows:

`.\target\release\dupels-cli.exe --help`

## Adding dupels to your PATH

For details, see:

- [How to add a directory to PATH in Linux/macOS](https://opensource.com/article/17/6/set-path-linux)

- [How to add to PATH on Windows](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/)

## Usage

`dupels --help` to display usage:

```text
$ dupels --help
Usage: dupels [OPTIONS] [FILE]

Arguments:
  [FILE]  Displays the name of files contained within a directory. If no operand is given, the contents of the current directory are displayed

Options:
  -a                               Include directory entries whose names begin with a dot (.)
  -r                               Generate the file names in a direcotry tree by walking the tree top-down.
                                   If the -d option is specified, walk to the depth specified, otherwise the default is depth of 2.
  -d, --depth <DEPTH>              Specifies the depth to generate file names during walk.
                                   The -d option implies the -r option.
  -s, --seperator <SEPERATOR>      Specify the seperator to use when listing the filenames [default: >--]
  -o, --omit                       Omit displaying files that are unique
      --max-threads <MAX_THREADS>  Specify the maximum number of threads to use.
                                   The default is the number of logical cores on the machine.
  -h, --help                       Print help
  -V, --version                    Print version
  -V, --version                    Print version
```

## Contributing

### Reporting Issues and Requesting Enhancements

If you’ve found a bug:

1. Head over to [issues](https://github.com/srdlj/dupels/issues)
2. Create a new issue with a clear title, detailed description, and steps to reproduce if applicable.
3. Label the issue appropriatly (ex: bugs labeled bugs, feature request labeled enhancements, etc.)

### Submitting Enhancements or Suggestions:

If you’d like to add a new feature:

1. Open an issue first to discuss your idea.

2. Once approved, submit a pull request with your implementation.

### Pull Requests

1. Fork the repository and clone it locally.

2. Create a new branch for your change.

3. Make your changes with clear, atomic commits.

4. Write tests and update documentation as needed.

5. Submit a pull request with a clear title and description.

6. Ensure all CI tests pass before requesting review.

## Development Set Up

It's recommended to use the devcontainer that's already set up for this project.

### Prerequisites

- [Docker](https://www.docker.com/get-started) installed and running.
- An editor or IDE that supports [Dev Containers](https://containers.dev/), such as [Visual Studio Code](https://code.visualstudio.com/) with the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers).

### Steps

1. Clone the repo
2. Open the project in your editor/IDE.
    - From your command palette select the option to "Reopen in Container"
3. Done!

## TODO's

- [ ] DupeLs-GUI! (maybe [egui](https://github.com/emilk/egui)?)
- [ ] Option to allow users to choose different cryptographic hash functions (SHA256, SHA1, etc.)
- [ ] Option to target popular formats: Audio -> wav, mp3, m4a, etc. Images -> jpg, png, gif, svg, etc. Video -> mp4, mov, etc.
- [x] Optimize recursive search
- [x] Introduce threads/parallel computing (checksum calculation causing bottlenecks)
- [x] Optimize MD5 checksum calculation (build from scratch)
- [x] Better error handling
