# skyline-rs-template

A template for writing skyline plugins for modding switch games using Rust and skyline-rs.

[Documentation for skyline-rs](https://ultimate-research.github.io/skyline-rs-template/doc/skyline/index.html)

## Prerequisites

* [Rust](https://www.rust-lang.org/install.html) - make sure rustup, cargo, and rustc (preferrably nightly) are installed.
* [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

## Setup

1. Install linkle and my fork of Xargo:
```sh
  # Install linkle
  cargo install --features=binaries --git https://github.com/MegatonHammer/linkle

  # Install Xargo
  cargo install --force --git https://github.com/jam1garner/xargo.git
```
2. Make a folder for you to store all of your plugins
```
mkdir plugins
cd plugins
```
3. Within your plugins folder, clone [rust-std-skyline-squashed](https://github.com/jam1garner/rust-std-skyline-squashed)
```sh
# Make sure you are inside of your plugins folder
git clone https://github.com/jam1garner/rust-std-skyline-squashed
```
Note: you only have to clone the `std` once to be shared between every plugin in the folder.

## Creating and building a plugin

1. From within the same folder as where you cloned `rust-std-skyline-squashed` clone this repo and rename it to match the
name of your plugin. To clone and rename all at once, use:
```
git clone https://github.com/jam1garner/skyline-rs-template [name of plugin]
```
2. Make sure you're inside the folder for your plugin:
```sh
cd [name of plugin]
```
3. There's a few places you'll need to rename your plugin. First in `Cargo.toml` near the top, change
```
name = "skyline-rs-template"
```
To a name suitable for your plugin. Next, go into `src/lib.rs` and edit the following line:
```rust
#[skyline::main(name = "module_name_test")]
```
to reflect what you want your module to be named on your console.

4. Lastly, to compile your plugin use the following command in the root of the project (beside the `Cargo.toml` file):
```sh
cargo nro
```
Your resulting plugin will be the `.nro` found in the folder ```
[plugin name]/target/aarch64-skyline-switch
```
To install (you must already have skyline installed on your switch), put the plugin on your SD at:
```
sd:/atmosphere/contents/[title id]/romfs/skyline/plugins
```
So, for example, smash plugins go in the following folder: ```
sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins
```

## Troubleshooting

**"Cannot be used on stable"**

First, make sure you have a nightly installed: ```
rustup install nightly
```
Second, make sure it is your default channel: ```
rustup default nightly
```
---
```
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/bin/cargo-nro.rs:280:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Make sure you are *inside* the root of the plugin you created before running `cargo nro`

Have a problem/solution that is missing here? Create an issue or a PR!

## Updating

For updating your dependencies such as skyline-rs:

```
cargo update
```

For updating your version of `rust-std-skyline-squashed`:

```
# From inside your plugins folder

rm -rf && git clone https://github.com/jam1garner/rust-std-skyline-squashed
```
