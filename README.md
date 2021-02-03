# Description

This is source code for the Rust-based operating system that is built following the steps outlined in this excellent set of blog posts, https://os.phil-opp.com/

# Getting it running

Run the following in succession to run the OS:

* If you have not done so already, install `rustup` and get a recent nightly build of Rust

```shell
brew install rustup
```

... then:

```shell
rustup override set nightly
```

* Check out this repo
  
```shell
git clone git@github.com:quephird/feos.git
```

* Fetch dependencies for this project

```shell
rustup component add llvm-tools-preview rust-src
```

* Download the `bootimage` utility:

```shell
cargo install bootimage
```

... then run it to create a new boot disk image
  
```
cargo bootimage
```

* Install `qemu` to allow for emulation of an x86 processor

```shell
brew install qemu
```

* Finally, run the project!

```shell
cargo run
```