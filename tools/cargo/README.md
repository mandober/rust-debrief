# Cargo

Cargo is Rust's project and package manager. 

`cargo` is a tool that allows Rust projects to declare their various dependencies and ensure that you’ll always get a repeatable build. It is part of the Rust toolchain that is installed with `rustup`. This project was bootstrapped with the following command:

`cargo new project`


## Set up a new project

The easiest way to set up a project to build a single library or an executable, by creating a new folder for the project is the `new` cargo subsommand. Before Rust [v.1.25](https://blog.rust-lang.org/2018/03/29/Rust-1.25.html) the default project was library (`--lib`), now it is executable (`--bin`), so you can leave out the `--bin` part (if building an executable). In fact, no matter which kind of project you start with you can change your mind later: executable project has a `src/main.rs` file, while a library has `src/lib.rs`.


```shell
# to initiate a new project (for executable) in the new subdir
cargo new --bin project_name

# to initiate a new project (for library) in the new subdir
cargo new --lib project_name
```


---
Links


Make explicit opt-out of target discovery neccessary for Rust 2018:
https://github.com/rust-lang/cargo/issues/5330

https://ashleygwilliams.github.io/a-very-brief-intro-to-rust/#5
