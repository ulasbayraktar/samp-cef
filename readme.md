# SAMP CEF - Remastered
This project embeds CEF into SA:MP expanding abilities to express yourself with beauty in-game interfaces using HTML / CSS / JavaScript.

It is **a FRAMEWORK** (or SDK), not something that you download and use. To be able to create you should have some webdev basics (JS / HTML / CSS).

## What you can do?
- Create browser views from a gamemode or from client-side plugins (C ABI).
- Place browsers on objects (with kind-of spatial sound)
- Send and receive custom defined events from / to clients.

## Available SA-MP Versions
- `0.3.7-R1`
- `0.3.DL-R1` - OnCefInitialize returning `true` but not working, wrong SA-MP addresses


## Install - For player
### Dependencies
- `SAMPFUNCS` - For the version you want (if you want for SA-MP 0.3.7, you should download `SAMPFUNCS 0.3.7`)
### Steps
- Download latest version (cef.zip)
- Put the files from the ZIP into the directory where GTA is installed.

## Install - For server
### Steps
- Download latest `server.dll` and `cef.inc`
- Place the file named `cef.inc` into the `pawno/plugins` directory, and put the file named `server.dll` into the `plugins` directory.
- Define `server.dll` as a plugin in the server configuration file.
- Define to `cef.inc` on gamemode.

## Crates
- `cef` - Rust wrappers around CEF C API.
- `cef-api` - Rust wrappers to build client plugins using CEF.
- `cef-interface` - example of a Rust plugin.
- `cef-sys` - bindings for CEF C API.
- `client` - client CEF plugin.
- `d3dx9` - bindings to DirectX SDK.
- `loader` - small loader that makes it work (should be named `cef.asi`).
- `messages` - protobuf messages to communicate with server on net.
- `network` - quinn glue (laminar like).
- `proto` - raw proto files.
- `renderer` - glue between CEF renderer process and main logic.
- `server` - server side plugin.
- 

## Building
### Dependencies
- [Rust compiler (nightly) with `i686-windows-pc-msvc` toolchain](https://rust-lang.org) (or nightly version)
- Prebuilt CEF with proprietary codes (if you wanna use streams). I had one for you in releases. (Client only)
- Environment variable `CEF_PATH` that points to `libcef.lib` (client only).
    - In powershell it's like `$env:CEF_PATH="C:/some/path"`
    - Then build

### Notes
If you get a linker error, you should change hard-coded links in the source code

- `client/build.rs` - path to DirectX SDK (default one) -  Default Path: `C:\Program Files (x86)\Microsoft DirectX SDK (June 2010)\Lib\x86`

If you get a build error, try build with nightly version

```sh
rustup toolchain install nightly-2022-11-06
cargo +nightly-2022-11-06 build --release
```

### Running Rust
and now
> ~~`cargo +nightly build --target i686-windows-pc-msvc --release`~~

```sh
rustup toolchain install nightly-i686
cargo +nightly-i686 build --release
```

also the client plugin can be built using OpenAL for sound ([rodio](https://crates.io/crates/rodio) by default). to do that compile the client without default features. for example:
```
cargo +nightly-i686 build --release --package client --no-default-features
```
to make it work you should place `openal.dll` as `sound.dll` in the `cef` folder. I do not remember what version is used exactly ... but I have it on the release page.

to build specific part you can add `--package <NAME>`

for example if you will try to build ALL crates on linux, you will get an error. so, pass  `--package server` to build only server on linux.

## CEF version

Current versions of CEF and Chromium:
`89.0.5+gc1f90d8+chromium-89.0.4389.40` `release branch 4389`

```
Date:             February 26, 2021

CEF Version:      89.0.5+gc1f90d8+chromium-89.0.4389.40
CEF URL:          https://bitbucket.org/chromiumembedded/cef.git
                  @c1f90d8c933dce163b74971707dbd79f00f18219

Chromium Version: 89.0.4389.40
Chromium URL:     https://chromium.googlesource.com/chromium/src.git
                  @2c3400a2b467aa3cf67b4942740db29e60feecb8
```

## Docs
- [docs/main_en.md](/docs/main_en.md)
- [docs/main_tr.md](/docs/main_tr.md)
- Also, check out wiki on github.

## Video examples
- [CEF Object #1](https://www.youtube.com/watch?v=Jh9IBlOKoVM)
- [Simple interfaces](https://www.youtube.com/watch?v=jU-O8_t1AfI)
- [Custom GTA interface](https://www.youtube.com/watch?v=qs7n8LoVYs4)
- [Voice chat](https://www.youtube.com/watch?v=vcyTjn3RJhs)
- [CEF Object #2](https://www.youtube.com/watch?v=6OnCSHKcOGU)

## Credit / Important Note
This project is a **fork project**. 
Original project: [https://github.com/Pycckue-Bnepeg/samp-cef/](https://github.com/Pycckue-Bnepeg/samp-cef/) 
