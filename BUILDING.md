# Building `xv6-rust`
## Installing `xv6` dependencies
You will first need to build standard xv6. To do so, we need the qemu emulator and riscv toolchains. 
For Ubuntu,
```bash
$ apt-get install git build-essential gdb-multiarch
$ apt-get install qemu-system-misc gcc-riscv64-linux-gnu binutils-riscv64-linux-gnu
```

## Install Rust
Install [`rustup`](https://rustup.rs/). This can be done by running
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Next, either restart your shell, or run `source $HOME/.cargo/env` to add
`rustup` and `cargo` to your path.

Now add the riscv target to the Rust compiler by running
```bash
rustup target add riscv64gc-unknown-none-elf
```
This allows building for the riscv platform.

Now, in the final project directory, if building from `git`, first checkout the
`main` branch.
```bash
git checkout main
```

Then you can build and run `xv6` like normal.
```bash
make
make qemu
```
