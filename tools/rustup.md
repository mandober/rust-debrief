# Rustup

- `rustup` is Rust platform installer and toolchain manager.
- It installs Rust, cargo, rustdoc
- homepage: https://www.rustup.rs
- github repo: https://github.com/rust-lang-nursery/rustup.rs




## Install rustup

### POSIX
Get and run rustup setup script: `curl https://sh.rustup.rs -sSf | sh`. This downloads and runs `rustup-init.sh`, which in turn downloads and runs the correct version of the `rustup-init` executable for your platform. Or see its help: `curl https://sh.rustup.rs -sSf | sh -s -- --help`


### Windows
Download and run [`rustup‑init.exe`][rustup‑init] .

- [`x86_64-pc-windows-msvc`][msvc]
  MSVC builds of rustup additionally require installation of `Visual C++ Build Tools 2015` if you don't have/want `Visual Studio 2015` installed. For `Visual Studio`, make sure to check the `C++ tools` option.
- [`x86_64-pc-windows-gnu`][gnu]

  No additional software necessary.



## Usage
Switch to the nightly as default channel: `rustup default nightly`




[rustup]: https://www.rustup.rs
[repo]: https://github.com/rust-lang-nursery/rustup.rs
[rustup‑init]: https://win.rustup.rs/x86_64
[gnu]: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-gnu/rustup-init.exe
[msvc]: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe