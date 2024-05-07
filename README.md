# dupels

Inspired by the `ls` command but groups files based on the MD5 checksum of each file's contents. The main use case of this tool is to identify duplicate files in a given path.

## Example

The following example shows all audio samples in a drumkit which are duplicates.
The `-d` flag specifies to go at most 2 subdirectories deep to find duplicates and the `-o` flag specifies to ignore all unique files.
The `---` seperator is a boundary for each duplicate group.

```bash
$ ./target/debug/dupels -d 2 -o ~/drum_kits/B-Wheezy 
/user/home/drum_kits/B-Wheezy/Vocals/Yeah2.wav
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/yeah_2.wav
---
/user/home/drum_kits/B-Wheezy/Vocals/hey6.wav
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/A!.wav
---
/user/home/drum_kits/B-Wheezy/Vocals/BWVox.wav
/user/home/drum_kits/B-Wheezy/Vocals/BStack.wav
---
/user/home/drum_kits/B-Wheezy/LiveDrums/ConcertBD1_3.wav
/user/home/drum_kits/B-Wheezy/LiveDrums/ConcertBD1_2.wav
---
/user/home/drum_kits/B-Wheezy/Vocals/Yup.wav
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/yup.wav
---
/user/home/drum_kits/B-Wheezy/Vocals/hey3.wav.asd
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/hey32_2.wav.asd
---
/user/home/drum_kits/B-Wheezy/Vocals/hey6.wav.asd
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/A!.wav.asd
---
/user/home/drum_kits/B-Wheezy/Vocals/SReddVox.wav
/user/home/drum_kits/B-Wheezy/Vocals/ShawtyRedd_3.wav
---
/user/home/drum_kits/B-Wheezy/Vocals/Yeah2.wav.asd
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/yeah_2.wav.asd
---
/user/home/drum_kits/B-Wheezy/Vocals/hey10.wav.asd
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/aye_2.wav.asd
---
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/Friday132.wav.asd
/user/home/drum_kits/B-Wheezy/VocalsMegaPack/Friday13.wav.asd
---
```

## Instillation

As of now, the project must be built from source.

`cargo build --release`

### Dependencies

- clap (``>= v3.0.0``)
- md5 (``>= 0.7.0``)

### Tests

You can run unit tests via `make tests`

## Usage

`dupels --help` to display usage:

```bash
$ ./target/release/dupels  --help
Usage: dupels [OPTIONS] [FILE]

Arguments:
  [FILE]  Displays the name of files contained within a directory. If no operands are given, the contents of the current directory are displayed

Options:
  -a                           Include directory entries whose names begin with a dot (.)
  -r                           Generate the file names in a direcotry tree by walking the tree top-down.
                               If the -d option is specified, walk to the depth specified, otherwise the default is depth of 8.
  -d, --depth <DEPTH>          Specifies the depth to generate file names during walk.
                               The -d option implies the -r option.
  -s, --seperator <SEPERATOR>  Specify the seperator to use when listing the filenames.
                               The default seperator is ">--" [default: ---]
  -o, --omit                   Omit displaying files that are unique
  -h, --help                   Print help
  -V, --version                Print version
```

## TODO's

- [ ] Optimize recursive search
- [ ] Optimize MD5 checksum calculation (build from scratch)
- [ ] Better error handling
