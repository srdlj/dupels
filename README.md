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

## Instillation

As of now, the project must be built from source.

`cargo build --release`

### Dependencies

- clap (``>= v3.0.0``)
- md5 (``>= 0.7.0``)

### Tests

`cargo test`

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

## Development Set Up

It's recommended to use the devcontainer set up for this project. Ensure you have docker and an editor/IDE which support Dev Container development. The devcontainer config can be found at `.devcontainer/devcontainer.json`.

## TODO's

- [ ] Option to allow users to choose different cryptographic hash functions (SHA256, SHA1, etc.)
- [ ] Option to target popular formats: Audio -> wav, mp3, m4a, etc. Images -> jpg, png, gif, svg, etc. Video -> mp4, mov, etc.
- [x] Optimize recursive search
- [x] Introduce threads/parallel computing (checksum calculation causing bottlenecks)
- [x] Optimize MD5 checksum calculation (build from scratch)
- [x] Better error handling
