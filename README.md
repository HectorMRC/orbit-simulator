# globe-rs

[![Continuos Integration](https://github.com/hectormrc/globe-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/hectormrc/globe-rs/actions/workflows/ci.yml)
[![Code Coverage](https://codecov.io/github/hectormrc/globe-rs/coverage.svg?branch=main&token=)](https://codecov.io/gh/hectormrc/globe-rs)
[![Crates.io: globe-rs](https://img.shields.io/crates/v/globe-rs.svg)](https://crates.io/crates/globe-rs)

A library for the management of geographic coordinates.

## About
This library provides a simple way to manipulate geographic coordinates while always maintaining consistent values. This means that, while the longitude does not take effect over any other ordinate, the latitude, on the other hand, may mutate the longitude depending on the magnitude of the overflow. For more information about this behavior, read the cargo-doc in this crate.