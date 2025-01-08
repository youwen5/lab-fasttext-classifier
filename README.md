# lab fasttext classifier

blah blah blah ill write description later.

## installation instructions

### agnostic way

Ensure you have a [Rust toolchain](https://rustup.rs/) installed (Rust stable
will do, no need for nightly), then `cargo install --release`.

### using Nix

If you already have Nix, no need to install a Rust toolchain. Run the command
to temporarily add the program to your shell.

```sh
nix run "nixpkgs#nix-output-monitor" -- shell
```

## running

First you need to train a model from the datasets in [./data](./data).
Once you have followed the above instructions to install the program, run

```sh
lab-intent-classifier train "<path-to-dataset>" "<path-to-output>"
```

For instance, using the already tokenized and processed data in
`./data/tokenized.txt`, you can train a model like so:

```sh
lab-intent-classifier train ./data/tokenized.txt ./model.bin
```

Then, you can run a query on the model like this:

```sh
lab-intent-classifier test ./model.bin "Where is the thermometer?"
```
