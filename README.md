# Desktop Sharkie

A little Gawr Gura desktop companion!

Very light weight, uses only 0-2% CPU and ~3mb RAM on a mid-range Windows laptop.

Check out this demo of it running:

[![Demo video](https://img.youtube.com/vi/E3FGzz8YJ5I/0.jpg)](https://www.youtube.com/watch?v=E3FGzz8YJ5I)

## Getting Started

To use desktop sharkie, you can either:
 - Download a [precompiled executable](https://github.com/K4rakara/desktop-sharkie/releases/) (Windows only for the time being)
 - Compile from source

### Compiling from Source (Windows)

First, you'll need to download & run [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019).
Once you're to the installation details screen, select "Desktop development
with C++" and wait for the install to finish.

Second, you'll want to install [rustup](https://rustup.rs/).

Finally, you just open CMD or PowerShell in this directory. Then, run
`cargo build --release`. **This will take a while.** Rust does heavy
optimizations to code, which makes it crazy lightweight and fast.

Just grab the file in `target/release/desktop-sharkie.exe`, and you're done!

### Compiling from Source (Linux)

First, install the `rustup` package with your package manager.

Then, open a terminal in this directory and run `cargo build --release`.

> Note: Linux is currently buggy. Even though its my daily driver, I wanted to
> get this out for the larger majority of users first, since I made it for
> Guras birthday.
> I'll be improving support for it in the coming weeks.

### Compiling from Source (MacOS)

I don't have any means of testing for MacOS, but I would assume its a similar
process to Linux, thanks to good ol' POSIX. If anybody has a Mac and would like
to contribute, I would be very appreciative.

