# rustup

- `rustup` is Rust platform installer and toolchain manager.
- It installs Rust, cargo, rustdoc
- homepage: https://www.rustup.rs
- github repo: https://github.com/rust-lang-nursery/rustup.rs

rustup installs The Rust Programming Language from the official release channels, enabling you to easily switch between stable, beta, and nightly compilers and keep them updated. It makes cross-compiling simpler with binary builds of the standard library for common platforms. If you are new to Rust consider running `rustup doc --book` to learn Rust.


## POSIX installation

Get and run rustup setup script:

```shell
curl https://sh.rustup.rs -sSf | sh
```

This downloads and runs `rustup-init.sh`, which in turn downloads and runs the correct version of the `rustup-init` executable for your platform.

Or check rustup help:

```shell
curl https://sh.rustup.rs -sSf | sh -s -- --help
```


## Windows installation

- Download and run [`rustup‑init.exe`][rustup‑init] .
- On Windows, Rust compiler can work with MSVC and GCC.
  - MSVC specific installation executable: [`x86_64-pc-windows-msvc`][msvc]
  - GCC specific installation executable: [`x86_64-pc-windows-gnu`][gnu]

MSVC builds of rustup additionally require installation of `Visual C++ Build Tools 2015` if you don't have `Visual Studio 2015` installed. For `Visual Studio`, make sure to check the `C++ tools` option.

No matter which version of rustup you install (msvc or gnu) you can make changes later.


## rustup usage
Common commands:

```shell
# add nightly
rustup 

# switch to the nightly as default channel
rustup default nightly

# open Rust book locally
rustup doc --book
```

## rustup toolchain
Many `rustup` commands deal with *toolchains*, a single installation of the Rust compiler. `rustup` supports multiple types of toolchains. The most basic track the official release channels:
- stable
- beta
- nightly

but `rustup` can also install toolchains from the official archives, for alternate host platforms, and from local builds.

Standard release channel toolchain names have the following form:

```
<channel>[-<date>][-<host>]

<channel> = stable|beta|nightly|<version>
<date>    = YYYY-MM-DD
<host>    = <target-triple>
```

`channel` is either a named release channel or an explicit version number, such as `1.8.0`. Channel names can be optionally appended with an archive date, as in `nightly-2017-05-09`, in which case the toolchain is downloaded from the archive for that date.

The host may be specified as a target triple. This is most useful for installing a 32-bit compiler on a 64-bit platform, or for installing the MSVC-based toolchain on Windows. For example:

```shell
rustup toolchain install stable-x86_64-pc-windows-msvc
```

For convenience, elements of the target triple that are omitted
will be inferred, so the above could be written:

```shell
rustup default stable-msvc
```

`rustup` can also manage symlinked local toolchain builds, which are often used to for developing Rust itself. For more information see:

```shell
rustup toolchain help link
```



[rustup]: https://www.rustup.rs
[repo]: https://github.com/rust-lang-nursery/rustup.rs
[rustup‑init]: https://win.rustup.rs/x86_64
[gnu]: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-gnu/rustup-init.exe
[msvc]: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe