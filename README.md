# Basic gitignore template

There are lots of good gitignore templates, but they're often just for one language. That's not great if you have a project that incorporates multiple languages and has a lot of custom files. It would be much better if you could start with a _generic_ gitignore and build on from there!

This repository contains:

- The `.gitignore` template
- And the `ginit` utility written in Rust that generates the template with a single command

## Usage

The easiest way to use this template is to use the provided `ginit` generator. This creates a `.gitignore` in the current folder. E.g. like this:

```bash
ginit
```

To download ginit, you can either download it from the [releases](https://github.com/Songtech-0912/basic-gitignore/releases/latest) or build it (assuming you have the Rust build tools):

```bash
cd ginit && cargo build --release && cp ./target/release/ginit ~/bin # or whatever location your global execs are in
```

However, if you would like to manually copy and paste the file, no problem - you can find it [here](./gitignore-template) as well.
