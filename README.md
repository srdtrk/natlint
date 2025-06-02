# natlint

Natlint is an open-source tool for linting [Solidity natspec comments](https://docs.soliditylang.org/en/develop/natspec-format.html).

## Installation

### Using `cargo`

```bash
cargo install natlint
```

### Using [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall)

```bash
cargo binstall natlint
```

### Using `nix`

```bash
nix shell github:srdtrk/natlint
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

## Usage in GitHub Actions

You can use Natlint in your GitHub Actions workflow by adding a step to run it. Here's an example of how to do that:

```yaml
name: natlint

on:
  pull_request:

jobs:
  natlint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: srdtrk/natlint@main
        with:
          include: 'contracts/**/*.sol'
```

### Inputs

You can pass inputs to the Natlint action using the `with` keyword. The available inputs are:

- `include`: A glob pattern to include files for linting. Defaults to `'**/*.sol'`.
- `exclude`: A glob pattern to exclude files from linting. Defaults to `''`.
- `config`: The path to the configuration file. Defaults to `'natlint.toml'`.
- `root`: The root directory to use for the glob patterns and configuration file. Defaults to `'.'`.
