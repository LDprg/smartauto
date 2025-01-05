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
  They primarly speed up development builds for rust (not needed for release builds).

# Building

Use `cargo build --release` to build the backend.
Use `bun run build` to build the frontend.

# Developing

Use `cargo build` to build the backend.

Use `cargo run --bin smartauto_backend` to run the backend.
Use `cargo run --bin test_plugin` to run a plugin.

Use `bun run dev -open`to run the frontend and its plugins

Don't forget to install pre-commit with `pre-commit install` before making any commits!

# Why SmartAuto

SmartAuto is very similar to homeassistant of the surface, but takes a lot of diffrent design choices, to fix many inconviences of homeassistant.
Modularity and perfomance as well as reliability are the main focus points of SmartAuto

### Modularity

SmartAuto is easily extendable and integratable by using grpc for a general interface between plugins, frontends, automisations and extenernal application.
This not only simplifies development process, since there is only one interface to maintain, it also allows easily extendability with custom software.
Due to the nature of grpc there is no restriction for which language to use as long as it supports grpc.

### Fast

The core of SmartAuto uses in Rust providing a quick backend. The frontend uses in svelte, which is one of the fastest frameworks.
SmartAuto uses ScyllaDB as database (swappable with Apache Cassandra, although not recommended), which is fast and easliy scaled.

The frontend is only a static generated site, so it doesn't need any special web server. We highly recommend caddy due to it good speed and easy configurability. Alternative you could use any other webserver (for example nginx).

### Scalleable

While SmartAuto is written pimarly for home automasiations, it's designed to scale almost indefinetly. Using ScallaDB allows db clustering, the frontend can also be distributed on multiple servers. One raspberry pi couldn't handle your homeautomisation load? Just grab a view and scale the thing up!

# Note

Currently this project is pre-alpha and native linux only.
Alot of thing will change!

THIS PROJECT ISN'T PRODUCTION READY IT'S TESTING ONLY.

# Planned features

A rough longterm todo-list:

- basic features working
- docker support
- plugin store
- support for backend clustering
- tools for clusters
