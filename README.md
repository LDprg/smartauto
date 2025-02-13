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

- [cranelift](https://github.com/rust-lang/rustc_codegen_cranelift)
- [mold](https://github.com/rui314/mold)
  They primarily speed up development builds for rust (not needed for release builds).

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

SmartAuto is very similar to homeassistant on the surface, but takes a lot of different design choices, to fix many inconveniences of homeassistant.
Modularity and performance as well as reliability are the main focus points of SmartAuto

### Simplicity

SmartAuto is designed to be easy and straight forward to use. Homeassistant for example complicates writing plugins (or integrations) alot by abstracting and heavily relying on python specific features. SmartAuto keeps interfaces simple with grpc and secure with its rust backend.

### Modularity

SmartAuto is easily extendable and integrable by using grpc for a general interface between plugins, frontend's, automation's and external application.
This not only simplifies development process, since there is only one interface to maintain, it also allows easily extendability with custom software.
Due to the nature of grpc there is no restriction for which language to use as long as it supports grpc.

### Fast

The core of SmartAuto uses in Rust providing a quick backend. The frontend uses in svelte, which is one of the fastest frameworks.
SmartAuto uses ScyllaDB as database (swappable with Apache Cassandra, although not recommended), which is fast and easily scaled.

The frontend is only a static generated site, so it doesn't need any special web server. We highly recommend caddy due to it good speed and easy configurability. Alternative you could use any other webserver (for example nginx).

### Scalable

While SmartAuto is written primally for home automatization, it's designed to scale almost indefinitely. Everything is designed to be able to work as cluster. ScyllaDB can be clustered by default, the frontend is just a statically generated site (could even be hosted locally) and the backend is written in a style that allows clustering.

# Note

Currently this project is pre-alpha and native linux only.
a lot of thing will change, don't expect anything!

THIS PROJECT ISN'T PRODUCTION READY IT'S TESTING ONLY.

The frontend will get a rust based implementation. The current one is just for testing, since I focus more on the backend for now.

# Contributions

Any contributions hightly appricated. Just fork the repo and make a PR. If you have questions about a idea you want to implement create a issue first.

# Planned features

A rough long-term todo-list:

- basic features working
- docker support
- plugin store
- support for backend clustering
- tools for clusters
