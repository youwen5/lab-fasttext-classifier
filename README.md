# lab fasttext classifier

using fasttext to figure out if a user is asking a question or issuing a
manipulation directive, in the context of a physics lab.

## installation instructions

As always, clone and `cd` into the repo's directory first. Then follow the
below instructions.

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

First you need to train a model from the datasets in [./data](./data). Once you
have followed the above instructions to install the program, run the help
command for more instructions:

```sh
lab-intent-classifier help
```

## example usage

First, let's split the dataset into a validation and training set.

```sh
lab-intent-classifier split-csv ./data/dataset.csv ./data/dataset.train.csv ./data/dataset.valid.csv
```

Then, let's process the training CSV to prepare for training.

```sh
lab-intent-classifier process ./data/dataset.train.csv ./dataset.tokenized
```

Now, train the model and output it to `./model.bin`.

```sh
lab-intent-classifier train ./dataset.tokenized ./model.bin
```

Now let's test the model!

```sh
lab-intent-classifier predict ./model.bin "where is the thermometer?"
```

Optionally, let's run a benchmark with our validation data, using 6 parallel
threads to speed it up.

```sh
lab-intent-classifier benchmark ./model.bin ./data/dataset.valid.csv 6
```
