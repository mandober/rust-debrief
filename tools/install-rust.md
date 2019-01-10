# Installing Rust

The usual way to install and manage Rust installations is with rustup, which is Rust's toolchain and versions manager.

```shell
# install rustup, which in turn installs rust, cargo, rustdoc
curl https://sh.rustup.rs -sSf | sh

# update rust (stable, beta, nightly) and rustup
rustup update

# update rustup only
rustup update self

# check Rust version
rustc --version
```
