# Carlae

Carlae is a minimal, Turing-complete, Python-like progamming language, written in Rust. It is still in development and does not support all Python syntax.

## Install

Install Carlae using [Cargo](https://rustup.rs/):

```sh
cargo install --git https://github.com/sharmuz/carlae.git
```

## Run a script

Create a file named `hello.carlae`:

```python
print "Hello, Carlae!"
```

Run it with:

```sh
carlae hello.carlae
```

Carlae takes a script file as an argument. It does not yet have an interactive prompt. For an overview of the language and planned features see the [language reference](docs/CARLAE.md).
