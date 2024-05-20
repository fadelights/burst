This project is meant to give a taste of the language.
Concepts shown here are not explained but will
be further taught in later chapters. Fret not if you don't
quite understand what exactly is going on!

# Misc. Notes
- The _prelude_ is the list of things that Rust automatically imports
into every Rust program.
- You can find dependencies (crates) for your project from
[crates.io](https://crates.io/). You can add them either by running
`cargo add` or directly editing your `Cargo.toml` file. The next time
`cargo build` is run, the dependencies will be automatically downloaded.
- Running the `cargo doc --open` command will build documentation
provided by all your dependencies locally and open it in your browser.
- The `Cargo.lock` file ensures reproducibility of builds.
