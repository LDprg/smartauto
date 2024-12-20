# smartauto
[![Rust](https://github.com/LDprg/smartauto/actions/workflows/rust.yml/badge.svg)](https://github.com/LDprg/smartauto/actions/workflows/rust.yml)
[![pre-commit](https://github.com/LDprg/smartauto/actions/workflows/pre-commit.yml/badge.svg)](https://github.com/LDprg/smartauto/actions/workflows/pre-commit.yml)

# Dependencies
Here a list of dependencies that might take more effort to install:
 - [https://www.rust-lang.org/tools/install](Rust)
 - [https://yew.rs/docs/next/getting-started/introduction](Yew)
 - [https://extism.org/docs/install/](Extism)
 - [https://github.com/sagiegurari/cargo-make#installation](cargo-make)
 - [https://pre-commit.com/#install](pre-commit)

# Building
Use `cargo make` in workspace folder for building each:
 - backend
 - frontend

# Developing
Use `cargo make run` in workspace folder for building each:
 - backend
 - frontend (live reload)
Use `cargo make clean` to clean the project (or workspace with all sub projects).

Don't forget to install pre-commit with `pre-commit install`!

# Note
Currently this project is pre alpha and native linux only.
Alot of thing will change!

THIS PROJECT IS NOT PRODUCTION READY IT IS TESTING ONLY.

# Planned features
This list might be incomplete and shouldn't be taken too serious:
 - docker support
