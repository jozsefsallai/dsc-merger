# DSC Merger

A command line utility for merging multiple Project Diva script files into one.
Useful for patching charts exported from Comfy Studio with additional commands
such as Challenge Time or lyric markers.

## Usage

### Interactive Mode

You can double-click the executable and follow the instructions on screen. You
can use the arrow keys to navigate the menus/options and <kbd>Enter</kbd> to
select or confirm them. For file paths, you can right click in the terminal
window to paste.

### Command Line Mode

Command line mode is recommended for more advanced users or just those who like
running things from the terminal. Here's an example command:

```
./dsc-merger -g "Future Tone" -i first_script.dsc -i second_script.dsc -p plaintext_script.dsc -o output.dsc
```

Command line mode can also be useful for scripting, batch processing, or any
other automation you might want to do.

You can check the [Options](#options) section of this README for a list of all
the available options that you can pass to the program. Passing ANY argument
will not run the program in interactive mode.

## Options

-   `-g` or `--game` - The game the encoded script files are made for. See the
    **Supported Games** section for a list of valid values for this argument.
    Defaults to `FT` (Future Tone).
-   `-i` or `--input` - A path to an encoded DSC file. Must be compatible with
    the game specified with the `-g` argument. You may provide multiple input
    files by specifying this argument multiple times.
-   `-p` or `--plaintext-input` - A path to a dumped/plaintext DSC file. Uses
    the same syntax that's used in editors such as [Open PD Script Editor][se-url].
    You may provide multiple plaintext input files.
-   `-s` or `--subtitle-input` - A path to an SRT, ASS or SSA file that contains
    timestamped lyrics. You may provide multiple but their pv_db will be
    separate. The file must have one of the following extensions: `.srt`, `.ass`
    or `.ssa`.
-   `--pv-id` - The ID of the PV that will be used to generate the entries for
    the lyrics in pv_db. Defaults to `0`.
-   `--english-lyrics` - Whether the specified lyrics are in English. This will
    make it use the `lyric_en` key instead of `lyric` for pv_db. Defaults to
    `false`.
-   `--max-lyric-length` - The maximum number of bytes that one line of lyrics
    can consist of. Defaults to `75`.
-   `--ct-start` - The time in `MM:SS.mmm` format at which the Challenge Time
    will start. This argument is optional and will be ignored if `--ct-end` or
    `--difficulty` are not specified.
-   `--ct-end` - The time in `MM:SS.mmm` format at which the Challenge Time will
    end. This argument is optional and will be ignored if `--ct-start` or
    `--difficulty` are not specified.
-   `--difficulty` - The difficulty of the chart (for Challenge Time). Valid
    arguments are `easy` and `normal` and is case-insensitive. This argument is
    optional and will be ignored if `--ct-start` or `--ct-end` are not
    specified.
-   `-o` or `--output` - The path to the output file. Defaults to `output.dsc`.
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
