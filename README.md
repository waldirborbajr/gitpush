
# GitPush - DevOps Automation Command Line Tool

[![Lint](https://github.com/waldirborbajr/gitpush/actions/workflows/linter.yaml/badge.svg)](https://github.com/waldirborbajr/gitpush/actions/workflows/linter.yaml) 
[![CI](https://github.com/waldirborbajr/gitpush/actions/workflows/ci.yaml/badge.svg)](https://github.com/waldirborbajr/gitpush/actions/workflows/ci.yaml) 
[![Release](https://github.com/waldirborbajr/gitpush/actions/workflows/release.yaml/badge.svg)](https://github.com/waldirborbajr/gitpush/actions/workflows/release.yaml) 
[![Security audit](https://github.com/waldirborbajr/gitpush/actions/workflows/audit.yaml/badge.svg)](https://github.com/waldirborbajr/gitpush/actions/workflows/audit.yaml)

## Overview

A simple command line tool written in Rust. It automates the process of adding all files, committing changes, and pushing them to the remote repository. This tool is designed to streamline the Git workflow, making it faster and more convenient.

## Usage

To use GitPush, follow these steps:

1. Navigate to the root directory of your Git repository in the terminal.

2. Run the following command:

```bash
gitpush
```

This command will execute the tool and perform the following operations:

1. Add all files recursively to the Git repository.
2. Commit all changes with a randomly generated commit message.
3. Push the changes to the remote repository (origin main branch).

## Installation

### Pre-Built Binary

Each release comes with pre-built binaries of several platforms. Grab it from [Github Releases](https://github.com/waldirborbajr/gitpush/releases).

### From source

Make sure you have Rust installed, then:

To build and install this, you'll need Rust and Cargo installed on your system. If you haven't already, you can install Rust by following the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can proceed with the following steps:

1. Clone the repository:

```bash
git clone https://github.com/waldirborbajr/gitpush.git
```

2. Navigate to the project directory:

```bash
cd gitpush
```

3. Build the project using Cargo:

```bash
cargo build --release
```

4. Install the binary:

```bash
cargo install --path .
```

## Dependencies

- `names`: This dependency is used to generate random commit messages. You can find more information about this crate [here](https://crates.io/crates/names).

- `std::os`: This module is part of the Rust standard library and is used for interacting with the operating system. It is used in this project for handling process exit codes.

## Contributing to GitPush

If you are interested in contributing to GitPush, we would love to have your help! You can start by checking out the [ open issues ](https://github.com/waldirborbajr/gitpush/issues) on our GitHub repository to see if there is anything you can help with. You can also suggest new features or improvements by opening a new issue.

To contribute code to Gitpush, you will need to fork the repository and create a new branch for your changes. Once you have made your changes, you can submit a pull request for them to be reviewed and merged into the main codebase.

## License

This project is released under the MIT License - see the [LICENSE](LICENSE) file for details.
