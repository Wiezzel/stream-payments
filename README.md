# Stream payments

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![Rust check](https://github.com/Wiezzel/stream-payments/actions/workflows/rust.yml/badge.svg)](https://github.com/Wiezzel/stream-payments/actions/workflows/rust.yml)

This pallet supports creating *streams* i.e. ongoing payments. Once a stream is opened,
on every block a specified amount of funds will be transferred from the origin account
to the given target account, until the stream is closed.

## Interface

### Config

* `MaxStreams: u32` – The maximum number of streams per account.

### Dispatchable functions

* `open_stream(origin, target, spend_rate)`  
   Open a new stream. From the next block on, on each block `spend_rate` will be transferred to the 
  `target` account. The stream can be closed by calling `close_stream`.
* `close_stream(origin, index)`  
   Close a stream. From the next block on, transfers will stop.

## Planned features

- [ ] Add an optional total spend limit for stream and reserve funds for limited streams.
- [ ] Use fixed stream identifiers instead of indices.
- [ ] Introduce fees/deposits for opening streams.
