# ppc750cl

Rust tools for working with the PowerPC 750CL family of processors.

### Rust crates

```shell
rustup components add rustfmt
cargo run --package ppc750cl-genisa
cargo build --release
```

### Python module

```shell
python -m venv env
source ./env/bin/activate
pip install maturin
maturin build -m ./disasm-py/Cargo.toml
```

### Instruction Set

For those unfamiliar with PowerPC, here are some basics.
- PowerPC 7xx is a family of RISC CPUs produced from 1997 to 2012.
  - They operate with 32-bit words and every instruction is 32-bits wide.
- This project focuses (only) on compatibility with the PowerPC 750CL.
  - This chip is famously packaged as codename "Broadway" for the Nintendo Wii.
  - Its predecessor PowerPC 750CXe is used in the Nintendo GameCube.
  - It adds a "paired-singles" SIMD unit and a bunch of other instructions.

### isa.yaml

The file [isa.yaml](./isa.yaml) contains a full definition of the PowerPC 750CL instruction set.

It powers the disassembler, assembler, and Rust/Python bindings code analysis tools.

Similarly to LLVM TableGen, the program `ppc750cl-genisa` generates a Rust file implementing an instruction decoder.
