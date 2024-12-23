# smartauto
[![Rust](https://github.com/LDprg/smartauto/actions/workflows/rust.yml/badge.svg)](https://github.com/LDprg/smartauto/actions/workflows/rust.yml)
[![pre-commit](https://github.com/LDprg/smartauto/actions/workflows/pre-commit.yml/badge.svg)](https://github.com/LDprg/smartauto/actions/workflows/pre-commit.yml)

# Dependencies
Here a list of dependencies that might take more effort to install:
 - [https://pre-commit.com/#install](pre-commit)
 - [https://www.rust-lang.org/tools/install](Rust)
 - NodeJs
 - pnpm
 - [https://github.com/bufbuild/protobuf-es](protobuf-es)
 - Svelte

# Building
Use `cargo make` to build the backend.
Use `pnpm build` to build the frontend.

# Developing
Use `cargo make run` to run the backend.
Use `cargo make clean` to clean the project.

Use `pnpm dev -open`to run the frontend

Don't forget to install pre-commit with `pre-commit install`!

# Note
Currently this project is pre alpha and native linux only.
Alot of thing will change!

THIS PROJECT IS NOT PRODUCTION READY IT IS TESTING ONLY.

# Planned features
This list might be incomplete and shouldn't be taken too serious:
 - docker support
