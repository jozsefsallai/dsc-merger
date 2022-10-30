# DSC Merger

A command line utility for merging multiple Project Diva script files into one.
Useful for patching charts exported from Comfy Studio with additional commands
such as Challenge Time or lyric markers.

## Usage

```
./dsc-merger -g "Future Tone" -i first_script.dsc -i second_script.dsc -p plaintext_script.dsc -o output.dsc
```

## Options

-   `-g` or `--game` - The game the encoded script files are made for. See the
    **Supported Games** section for a list of valid values for this argument. This
    argument is required.
-   `-i` or `--input` - A path to an encoded DSC file. Must be compatible with the
    game specified with the `-g` argument. You may provide multiple input files by
    specifying this argument multiple times.
-   `-p` or `--plaintext-input` - A path to a dumped/plaintext DSC file. Uses the
    same syntax that's used in editors such as [Open PD Script Editor][se-url].
    You may provide multiple plaintext input files.
-   `-o` or `--output` - The path to the output file. This argument is required.
-   `-v` or `--verbose` - Enables verbose logging. Useful for debugging.
-   `--dump` - Dumps a plaintext version of the merged DSC output. Useful for
    debugging.

## Supported Games

The aliases are case-insensitive.

-   `Future Tone` (aliases: `ft`, `futuretone`)
-   `F`
-   `F2nd` (aliases: `f 2nd`, `f2`)
-   `X`

## Building

The application is written in Rust. You can build it using `cargo`:

```
cargo build
./target/debug/dsc-merger -h
```

You may also use the following command to run the program without building it:

```
cargo run -- -i first_script.dsc -i second_script.dsc -o output.dsc
```

## Credits

-   [Open PD Script Editor][se-url] - For the plaintext format used in the
    plaintext input files as well as the list of opcodes used by the DSC format.

## License

MIT.

[se-url]: https://github.com/nastys/Open-PD-Script-Editor
