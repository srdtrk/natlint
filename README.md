# natlint

Natlint is an open-source tool for linting [Solidity natspec comments](https://docs.soliditylang.org/en/develop/natspec-format.html).

## Installation

You can install Natlint using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```bash
cargo install natlint
```

## Usage

First initialize a configuration file, if you don't have one:

```bash
natlint init
```

This will create a `natlint.toml` file in the current directory with the default settings. See the [commented config file](./natlint.toml) for more details on the available options.

Then run Natlint with one or more Globs as arguments. For example, to lint all files inside contracts directory, you can do:

```bash
natlint run --include 'contracts/**/*.sol'
```

Run Natlint with the `-h` option to see all available options.

## Configuration

You can use a `natlint.toml` file to configure Natlint. You can also specify the path to the configuration file with the `--config` option.

```bash
natlint run --config path/to/natlint.toml -i 'contracts/**/*.sol'
```

To generate a default configuration file, run:

```bash
natlint init
```

## Inline Configuration

You can use comments in the source code to configure Natlint in a given line.

```solidity
// natlint-disable-next-line
function foo() public {
    // ...
}

/// @notice This function does something important
// natlint-disable-next-line RuleName1,RuleName2
function bar() public {
    // ...
}
```
