# hammer
Safe Network - simple front GUI for the CLI

## Install

  Download your prefered binary execuable.
  There is one for Windows named "hammer.exe" and another for Linux named "hammer".

## Basic Usage
	$ hammer

### Optional: language code ISO639-1, defaults to en.
	$ hammer el
	$ hammer en
	$ hammer ko

#### Options:
    -h, --help          display help and exit
    -V, --version       output version information and exit

## Description

  This is a simple front end GUI for the Safe Network and makes calls to the Safe Network Command Line Interface (CLI).

  For more about the Safe Network see:

  <https://safenetwork.tech/>

  <https://safenetforum.org/>

  <https://github.com/maidsafe/>

## Build
  The src is the full code - with translations, where the root is truely a template that spawns that.
  What is visible then is the src needed to create the binaries.

  You need the folder ./src/ and the Cargo.toml and from the folder containing those run cargo.
  Required is cargo, which you can get from https://doc.rust-lang.org/cargo/getting-started/installation.html

  On Linux then build is simply

	$ cargo build --release
	$ cargo doc --no-deps

  What is avaliable then is the result of both
	
	cargo build --target=x86_64-unknown-linux-gnu --release
	cargo build --target=x86_64-pc-windows-gnu --release

  but you might want to also try your own platform as

	cargo build --target=aarch64-unknown-linux-gnu --release
	cargo build --target=x86_64-apple-darwin --release
	cargo build --target=x86_64-pc-windows-msvc --release 

  which gave error from Linux here as "failed to run custom build command for fltk-sys"

  and then 32bit perhaps requires the architecture for building that

	cargo build --target=i686-pc-windows-gnu
	cargo build --target=i686-pc-windows-msvc
	cargo build --target=i686-unknown-linux-gnu

