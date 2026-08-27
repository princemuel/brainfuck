# Brainfuck

An experimental Brainfuck compiler that generates native executables using the LLVM compiler framework.

## Language overview

The Brainfuck programming language is a _Turing-complete language_ created by Urban Müller.
The language only consists of 8 operators, yet with the 8 operators, `<>+-[],.`
you are capable of writing almost any program you can think of.

The idea behind `brainfuck` is memory manipulation. Basically, you are given
an array of 30,000 1byte memory blocks. The array size is actually dependent upon
the implementation used in the compiler or interpreter, but standard brainfuck
states 30,000. Within this array, you can increase the memory pointer,
increase the value at the memory pointer, etc.

```brainfuck
> = increases the memory pointer, or moves the pointer to the right by 1 block.
< = decreases the memory pointer, or moves the pointer to the left by 1 block.
+ = increases the value stored at the block pointed to by the memory pointer
- = decreases the value stored at the block pointed to by the memory pointer
[ = like c while(cur_block_value != 0) loop.
] = if block currently pointed to's value is not zero, jump back to [
, = like c getchar(). input 1 character.
. = like c putchar(). print 1 character to the console
```

### Rules

- Any arbitrary character besides the 8 listed above should be ignored by the
  compiler or interpretor. Characters besides the 8 operators should be con-
  sidered comments.

- All memory blocks on the "array" are set to zero at the beginning
  of the program. And the memory pointer starts out on the very left-most
  memory block.

- Loops may be nested as many times as you want. But all `[` must have a corresponding `]`.

## Working Locally

### Prerequisites

| Requirement    | Version                   |
| -------------- | ------------------------- |
| Rust (nightly) | see `rust-toolchain.toml` |
| LLVM           | 22.x                      |
| `llvm-config`  | on `$PATH`                |

**Arch Linux:**

```sh
sudo pacman -Syu llvm clang lld lldb
```

**macOS (Homebrew):**

```sh
brew install llvm@22
```

```sh
export PATH="$(brew --prefix llvm@22)/bin:$PATH"
```

**Ubuntu / Debian:**

```sh
wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh && sudo ./llvm.sh 22 all
```

**Fedora/CentOS:**

```sh
sudo dnf update --refresh
```

```sh
sudo dnf install llvm-devel clang-devel lld lldb clang-tools-extra
```

Verify the installation was a success:

```sh
llvm-config --version   # should print 22.x.x
```

### Building

```sh
git clone https://github.com/princemuel/brainfuck && cd brainfuck
```

```sh
# create a debug build
cargo build
```

```sh
# create a release build
cargo build --release
```

### Testing

```sh
# All tests
cargo test

# A specific module
cargo test --lib -- lexer::tests
cargo test --lib -- parser::tests
cargo test --lib -- codegen::tests

# With stdout captured
cargo test -- --nocapture
```

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

## Resources used
