# Stream payments

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![Rust check](https://github.com/Wiezzel/stream-payments/actions/workflows/rust.yml/badge.svg)](https://github.com/Wiezzel/stream-payments/actions/workflows/rust.yml)

This pallet supports creating *streams* – ongoing payments. Once a stream is opened,
on every block a specified amount of funds will be transferred from the creator's account
to the given target account until the stream is closed.

## Interface
TBD 

## TODO:

### Maintenance

- [ ] Fill in README
- [ ] Add docstrings
- [x] Compute proper weights
  - [x] Extrinsics
  - [x] Hooks
- [x] Write some tests

### Features

- [ ] Auto-close streams when account runs out of funds (?)
- [ ] Add an optional total spend limit for stream and reserve funds for limited streams.
- [ ] Disallow origin == target (that's most probably user's mistake).

License: Apache-2.0
