[![docs.rs](https://docs.rs/c_bridge/badge.svg)](https://docs.rs/c_bridge)
[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/c_bridge.svg)](https://crates.io/crates/c_bridge)
[![Download numbers](https://img.shields.io/crates/d/c_bridge.svg)](https://crates.io/crates/c_bridge)
[![Travis CI](https://travis-ci.org/KizzyCode/c_bridge.svg?branch=master)](https://travis-ci.org/KizzyCode/c_bridge)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/c_bridge?svg=true)](https://ci.appveyor.com/project/KizzyCode/c-bridge)
[![dependency status](https://deps.rs/crate/c_bridge/0.5.0/status.svg)](https://deps.rs/crate/c_bridge/0.5.0)

# About `c_bridge`
This crate provides some data structures and abstractions to create clean Rust <-> C FFI interfaces


# Dependencies
If you build the crate with `test_build` enabled, the `cc`-crate is required to build and link the
necessary C test code. By default, the crate is dependency less.