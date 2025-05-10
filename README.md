![build](https://github.com/srdlj/dupels/actions/workflows/rust.yml/badge.svg) [![Coverage Status](https://coveralls.io/repos/github/srdlj/dupels/badge.svg?branch=main)](https://coveralls.io/github/srdlj/dupels?branch=main) ![GitHub Release](https://img.shields.io/github/v/release/srdlj/dupels)
# dupels

Inspired by the `ls` command but lists directory contents grouped by their checksum (MD5, options for other cryptographic hash functions coming soon). The main use case of this tool is to identify duplicate files in nested directories efficiently.

## Example

The following example shows all audio samples across 4 drumkits in the current directory `drum_kits` which are duplicates.
The `-d` option specifies to go at most 3 subdirectories deep to find files and the `-o` option omits all files with
a unique checksum.

The `>--` seperator is a boundary for each checksum group (in this case groups of duplicates as we've omited all unique files).

```bash
$ dupels -d 3 -o drum_kits
drum_kits/kit_1/open hat/oh (wod).wav
drum_kits/kit_1/open hat/oh (baby pluto).wav
>--
drum_kits/kit_3/REAL TRAPPER PERCZ/SF RT PERC 20.wav
drum_kits/kit_0/Hi Hats/Dp Beats- Hi Hat (3).wav
>--
drum_kits/kit_1/claps/clap (baby pluto).wav
drum_kits/kit_1/claps/clap (wod).wav
>--
drum_kits/kit_3/REAL TRAPPER SOUNDFONTZ/ZSF_Brass_Ensemble_SE.sf2
drum_kits/kit_2/VST Presets/Soundfonts/Brass Ensemble SE.sf2
>--
drum_kits/kit_1/808s/classic zay.wav
drum_kits/kit_0/808s/Dp Beats- 808(9).wav
>--
drum_kits/kit_3/REAL TRAPPER PERCZ/SF RT PERC 47.wav
drum_kits/kit_3/REAL TRAPPER PERCZ/SF RT PERC 37.wav
>--
drum_kits/kit_2/VST Presets/Other/OFFICIAL D. RICH PIZZICATO.sf2
drum_kits/kit_2/VST Presets/Soundfonts/Piccolo (2).sf2
drum_kits/kit_2/VST Presets/Soundfonts/Pizzicato_1.sf2
>--
drum_kits/kit_3/REAL TRAPPER SOUNDFONTZ/Pizzicato Strings.sf2
drum_kits/kit_3/REAL TRAPPER SOUNDFONTZ/Piccolo (5).sf2
drum_kits/kit_2/VST Presets/Soundfonts/Piccolo (5).sf2
>--
drum_kits/kit_1/kicks/kick (baby pluto).wav
drum_kits/kit_1/kicks/kick (wod).wav
>--
drum_kits/kit_3/REAL TRAPPER CLAPZ/SF RT CLAP 2.wav
drum_kits/kit_0/Claps/Dp Beats- Clap(1).wav
drum_kits/kit_0/Claps/Dp Beats- Clap (5).wav
>--
drum_kits/kit_1/sfx/ripsquadd riser 2.wav
drum_kits/kit_0/FX/Dp Beats- Drop (3).wav
>--
drum_kits/kit_2/VST Presets/Soundfonts/Synths (2).SF2
drum_kits/kit_2/VST Presets/Soundfonts/Synths (1).SF2
>--
drum_kits/kit_3/REAL TRAPPER SOUNDFONTZ/Orchestra Hits.sf2
drum_kits/kit_2/VST Presets/Soundfonts/Orchestra Hits.sf2
```

## Understanding False Positives and False Negatives

Due to the nature of using checksum analysis for detecting duplicate files, **false negatives can occur**. For example, two MP3 files might sound identical, but still have different checksums if one is encoded at 128 kbps and the other at 320 kbps. Despite being perceptually the same, their binary differences result in unique checksums. On the other hand, false positives, where two files with different binary representations produce the same checksum, are extremely rare. The likelihood of this happening is about 1 in 2^128 for MD5 (unless the files are deliberately engineered to cause a collision). ***As a disclaimer, this tool is to help aid with productivity and file management, NOT to dictate definitive decissions.***

## Installation

### Linux

You can download the latest release from the [GitHub Releases page](https://github.com/srdlj/dupels/releases) and extract the appropriate archive for your system:

```bash
wget https://github.com/srdlj/dupels/releases/latest/download/dupels-linux.tar.gz
tar -xzf dupels-linux-<version>.tar.gz
cd dupels-linux-<version>
./dupels-cli --help
```

Or, for the zip archive:

```bash
wget https://github.com/srdlj/dupels/releases/latest/download/dupels-linux.zip
unzip dupels-linux-<version>.zip
cd dupels-linux-<version>
./dupels-cli --help
```

### macOS

You can download the latest release from the [GitHub Releases page](https://github.com/srdlj/dupels/releases) and extract the appropriate archive for your system:

```bash
curl -LO https://github.com/srdlj/dupels/releases/latest/download/dupels-macos.tar.gz
tar -xzf dupels-macos.tar.gz
cd dupels-macos
./dupels-cli --help
```

Or, for the zip archive:

```bash
curl -LO https://github.com/srdlj/dupels/releases/latest/download/dupels-macos.zip
unzip dupels-macos.zip
cd dupels-macos
./dupels-cli --help
```

### Windows

Download the Windows release from the [GitHub Releases page](https://github.com/srdlj/dupels/releases):

With zip:

```PowerShell
Expand-Archive -Path .\dupels-windows.zip -DestinationPath .\dupels-windows
cd .\dupels-windows
.\dupels-cli.exe --help
```

Or, with tar:

```
tar -xzf .\dupels-windows.tar.gz
cd .\dupels-windows
.\dupels-cli.exe --help
```

### Building from Source

Ensure you have [Rust](https://rustup.rs/) installed, then run:

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
- An editor or IDE that supports [Dev Containers](https://containers.dev/), such as Visual Studio Code with the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers).

### Steps

1. Clone the repo
2. Open the project in your editor/IDE.
    - From your command palette select the option to "Reopen in Container"
3. Done!

## License

By contributing, you agree that your contributions will be licensed under the [LICENSE](https://github.com/srdlj/dupels/blob/main/LICENSE) file in the root of this repository.

## TODO's

- [ ] DupeLs-GUI (maybe [egui](https://github.com/emilk/egui)?)
- [ ] Option to allow users to choose different cryptographic hash functions (SHA256, SHA1, etc.)
- [ ] Option to target popular formats: Audio -> wav, mp3, m4a, etc. Images -> jpg, png, gif, svg, etc. Video -> mp4, mov, etc.
- [x] Optimize recursive search
- [x] Introduce threads/parallel computing (checksum calculation causing bottlenecks)
- [x] Optimize MD5 checksum calculation (build from scratch)
- [x] Better error handling
