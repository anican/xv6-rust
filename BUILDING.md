# Building `xv6-rust`
## Installing `xv6` dependencies
...
TODO (qemu)

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
`xv6-rust` branch.
```bash
git checkout xv6-rust
```

If building on `attu`, add the riscv tools to your path
```bash
export PATH="/cse/courses/cse451/21au/riscv/bin:$PATH"
```

Then you can build and run `xv6` like normal.
```bash
make
make qemu
```
