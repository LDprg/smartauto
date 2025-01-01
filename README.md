# SmartAuto

[![Rust](https://github.com/LDprg/smartauto/actions/workflows/rust.yml/badge.svg)](https://github.com/LDprg/smartauto/actions/workflows/rust.yml)
[![Typescript](https://github.com/LDprg/smartauto/actions/workflows/typescript.yml/badge.svg)](https://github.com/LDprg/smartauto/actions/workflows/typescript.yml)
[![pre-commit](https://github.com/LDprg/smartauto/actions/workflows/pre-commit.yml/badge.svg)](https://github.com/LDprg/smartauto/actions/workflows/pre-commit.yml)

# Dependencies

These dependencies should be installed first:

- [pre-commit](https://pre-commit.com/#install)
- [Rust nightly](https://www.rust-lang.org/tools/install)
- [bun](https://bun.sh)
- [protobuf-es](https://github.com/bufbuild/protobuf-es)

It's also recommended to install:

- [crangelift codegen](https://github.com/rust-lang/rustc_codegen_cranelift)
- [mold](https://github.com/rui314/mold)
  They primarly speed up development rust build process.

# Building

Use `cargo build` to build the backend.
Use `bun run build` to build the frontend.

# Developing

Use `cargo run --bin smartauto_backend` to run the backend.
Use `cargo run --bin test_plugin` to run a plugin.

Use `bun run dev -open`to run the frontend

Don't forget to install pre-commit with `pre-commit install` before making any commits!

# Why SmartAuto

SmartAuto tries to be a homeautomisation software.
Here some important design decisions:

### Modularity

SmartAuto is easily extendable and integratable by using grpc for a general interface between plugins, frontends, automisations and extenernal application.
This not only simplifies development process, since there is only one interface to maintain, it also allows easily extendability with custom software.
Due to the nature of grpc there is no restriction for which language to use as long as it supports grpc.

### Fast

The core of SmartAuto uses in Rust providing a quick backend. The frontend uses in svelte, which is one of the fastest frameworks.
SmartAuto uses ScyllaDB as database (swappable with Apache Cassandra, although not recommended), which allows easy clusters and backups.

The frontend is only a static generated site, so it doesn't need any special web server. We highly recommend caddy due to it good speed and easy configurability. Alternative you could use any other webserver (for example nginx).

# Note

Currently this project is pre-alpha and native linux only.
Alot of thing will change!

THIS PROJECT ISN'T PRODUCTION READY IT'S TESTING ONLY.

# Planned features

A rough longterm todo-list:

- basic features working
- docker support
- plugin store
