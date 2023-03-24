# nen

__nen__ is a simple programming language written in Rust.

**IMPORTANT: nen is currently in very early development, and is subject to change without warning.**

## Getting Started

To get started with __nen__, you need to have Rust installed on your system.

You can install Rust and its associated toolchain by following instructions [here](https://rustup.rs/).

Once you have Rust installed, you can clone the __nen__ repository to your local machine.

You can then build the __nen__ binary by running the following command inside the __nen__ directory:

```bash
$ cargo build --release
```

The binary, which will be generated in the `target/release/` directory, can be run using the following command:

```bash
./target/release/nenc examples/hello.nen
```

This will compile your `nen` source into a `out.nenc` file, which can be run with the following command:

```bash
./target/release/nenc --interpret out.nenc
```

## Examples

__nen__ comes with several example programs to help you understand the language and try it out. These can be found in the `examples/` directory.

## Contributing

Contributions to __nen__ are welcome, and encouraged! If you find a bug, or have a feature request, please open an issue in the GitHub repository [here](https://github.com/morrig-n/nen/issues).

## License

__nen__ is licensed under the MIT License. Please see the [LICENSE](LICENSE) file for more information.
