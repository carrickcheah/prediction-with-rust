# RUST

## Session 1

### Goals

- [x] Set up the tools
    - [x] rustc
    - [x] cargo
    - [x] rust analyzer
    - [x] ide -> cursor/vscode

- [x] Write a basic Hello World app with Cargo
- [x] Compile and run it

- [ ] Download external CSV file to disk
- [ ] Load file from disk into memory
- [ ] Prepare the data
- [ ] Train an XGBoost model with this data
- [ ] Push this model to an AWS S3 bucket (model registry)


### 1. Setting up the tools

1. Install Rust using rustup (the official Rust installer):
   - **Windows**: Visit https://rustup.rs and download/run rustup-init.exe
   - **macOS/Linux**: Run in terminal:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

     Further information for Windows users is [here](https://rust-lang.github.io/rustup/installation/windows-msvc.html)

2. Test the installation worked
    ```bash
   rustc --version
   cargo --version
   ```

3. Install the `rust-analyzer` extension.

### 2. Write, compile and run a Hello World program

```bash
cargo new house-price-predictor
cd house-price-predictor

# compiles and runs the program
cargo run
```

After this last command, there is a new binary file inside `target/debug/house-price-predictor`
This is a standalone binary, you can run on your computer. No need for `rustc` or `cargo` anymore.

```bash
# Run your debug binary
./target/debug/house-price-predictor
```

You generate the final optimized binary by running
```bash
cargo new --release
./target/release/house-price-predictor
```

#### Makefiles to the rescue
I recommend creating a Makefile at the root of your project to encapsulate
potentially long, repetive commands, into simple `make` instructions.

   
### How to download an external file with `reqwest`?

- Install reqwest `cargo add reqwest` with the `blocking` feature, because by default is an `async` operation, but we don't care about async and we just want to make it work `sync`.

- reqwest::blocking::get(url) returns a Result enumeration

```rust
enum Result<T, E> {
    Ok(T)
    Err(E)
}
```

