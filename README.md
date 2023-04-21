# oursh
[![Documentation](https://docs.rs/oursh/badge.svg)](https://docs.rs/oursh)
[![CI](https://github.com/nixpulvis/oursh/actions/workflows/ci.yml/badge.svg?branch=develop)](https://github.com/nixpulvis/oursh/actions/workflows/ci.yml)
[![Dependencies](https://deps.rs/repo/github/nixpulvis/oursh/status.svg)](https://deps.rs/repo/github/nixpulvis/oursh)

This shell should be both POSIX compatible and yet modern and exciting. Fancy
features should not be prevented by POSIX compatibility. This will effect the
design of the shell.

The name of the shell is `oursh` which is both somewhat unique, and memorable.
It's also a nice name to play with pseudo-satirical themes... right comrade?
It's short (ish) and sneakily fits `rs` in it, which is the extension of Rust
programs, the language this will be written in.

## Features

- [ ] [POSIX compatibility](https://github.com/nixpulvis/oursh/milestone/1)
    - [x] Simple commands `ls`
    - [ ] Quotes (#28) `echo "foo"; echo 'bar'`
    - [x] Assignment `LOG=trace cargo run`
    - [x] Variables `echo $foo`
    - [ ] Special variables ($54) `echo $?; echo $1`
    - [x] Boolean status syntax `! true && false || true`
    - [x] Conditionals `if ; then ; elif ; then ; else ; fi`
    - [x] Compound commands `{ ls; date; }`
    - [ ] Subshells `\$(sleep 1; date)`
    - [x] Background jobs `{ sleep 1; date; }& date`
    - [x] Redirection `date > now.txt`
    - [ ] Pipes `ls | wc -l`
- [ ] Shebang block programs
    - [ ] Alternate syntax `{# ...}`
    - [ ] Hashlang syntax `{#lang; ...}`, i.e. `{#posix ls}`
    - [x] Shebang syntax `{#!/usr/bin/env ruby; puts :sym}`
- [ ] bash/zsh autocomplete compatibility
    - [ ] Command completion
    - [ ] Path completion
    - [ ] Variable completion
    - [ ] Job completion
    - [ ] Syntax completion
    - [ ] `man` / `-h` / `--help` parsing
- [ ] Multi-line input
- [ ] Modern scripting language
    - [ ] Macros
    - [ ] Types
    - [ ] Higher-order functions
    - [ ] Threading?
- [ ] Obfuscated strings (`!'password'!`)
- [ ] Time every command
- [ ] mosh like remote session support
- [ ] Smart history, sync'd across devices
- [ ] Pipe old commands without rerunning
- [ ] Package manager
- Sane defaults
- Fast

## Releases, Tags, and Branches

The `master` branch will run both the [`release`][action-release] and
[`ci`][action-ci] actions, whereas `develop` only runs `ci`. All pull requests
into `master` and release tagged (i.e. vX.Y.Z) pushes also trigger `release`
builds.

## Usage

While this project is in early stages, there are no OS packages to use.
However, you can compile and run directly from source easily. Just ensure you
have [`rustup`][rustup] installed.

```sh
cargo build
cargo run
```


## Testing

We have four kinds of tests in this program. Crate unit tests, Executable unit
tests, subprocess based integration tests, and documentation tests.

```sh
# Run all tests.
cargo test
```


## Previous Work

I've been using [`fish`][fish] as my main shell for a few years now. Fish
inspires a lot of the modern syntax.

POSIX compatibility comes from my desire to use this shell as my `chsh -s ...`
shell on [Arch Linux][arch]. See the full POSIX reference for more information.

Some of the shebang language interoperation was inspired by my time with the
Northeastern University PL group, and generally from writing [Racket][racket].
The beauty of of merging the UNIX style `#!...` with Racket's `#lang ...` here
is very exciting to me. I might just _have_ to make a `{#lang ...}` shortcut
for Racket!

I've built and wrote a few things about shells before:

- [`rush`][rush] A glorified homework assignment for computer architecture
- [`shell.py`][shell.py] My submission for computer organization a8
- [Building a Shell - Part 1][basp1] Start of this project
- [Building a Shell - Part 2][basp2] `program` module intro


## [POSIX Reference][posix]

See the following sections for building the POSIX `sh` compliant program
language, and interactive terminal based REPL.

- 3§2 Shell Command Language
    - 10.2 Shell Grammar Rules
- 2§2.5 Standard I/O Streams
- 3§1.6 Built-In Utilities
- 3§1.4 Utility Description Defaults
- 2§2.3 Error Numbers
- 1§11 General Terminal Interface
- 2§2.4 Signal Concepts


## Implementation

This shell will be written in Rust with minimal dependencies. Notably
[termios][termios] and [libc][libc] will likely be used. The parsing library
will be [lalrpop][lalrpop], which should support the syntax we want somewhat
easily, though grammar's in general can be a tricky beast.

We will want to create a few internal modules for the shell.

**This design is subject to change.**

- `process` - sub-process execution management.
- `program` - parser and interpreter for the syntax of the shell.
    - `posix` - POSIX (`sh`-like) syntax.
    - `modern` - Modified syntax for supporting "modern" features, like lambdas.
- `repl` - syntax aware, read eval print loop for an underlying terminal.
    - `history` - records previous execution to a shared DB.
    - `completion` - searches for autocompletions based on partial syntax.
        - `bash` - bash completion support.
        - `zsh` - zsh completion support.
        - `parse` - dynamic completion generation, from `man` for example.
    - `sync` - remote session and DB synchronization.
- `invocation` - loading for `.ourshrc` and others.
- `package` - simplistic package manager support (builtin function).

---

_Current modules as of: 2018-10-14_

```
 oursh : crate
 ├── job : public
 ├── program : public
 │   ├── basic : public
 │   └── posix : public
 │       ├── ast : public
 │       │   └── tests : private @ #[cfg(test)]
 │       ├── builtin : public
 │       └── lex : public
 │           └── tests : private @ #[cfg(test)]
 ├── repl : public
 │   ├── completion : public @ #[cfg(feature = "completion")]
 │   └── history : public @ #[cfg(feature = "history")]
 └── tests : private @ #[cfg(test)]└── tests : private @ #[cfg(test)]
```


[documentation]: https://nixpulvis.com/oursh/oursh
[action-release]: https://github.com/nixpulvis/oursh/blob/master/.github/workflows/release.yml
[action-ci]: https://github.com/nixpulvis/oursh/blob/master/.github/workflows/ci.yml
[rustup]: https://github.com/rust-lang-nursery/rustup.rs
[posix]: http://pubs.opengroup.org/onlinepubs/9699919799/
[termios]: https://crates.io/crates/termios
[libc]: https://crates.io/crates/libc
[lalrpop]: https://github.com/lalrpop/lalrpop
[fish]: https://github.com/fish-shell/fish-shell
[arch]: https://www.archlinux.org/
[racket]: https://racket-lang.org/
[rush]: https://github.com/nixpulvis/rush
[shell.py]: /doc/shell.py
[basp1]: https://nixpulvis.com/ramblings/2018-07-11-building-a-shell-part-1
[basp2]: https://nixpulvis.com/ramblings/2018-10-15-building-a-shell-part-2

