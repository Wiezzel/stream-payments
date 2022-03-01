# Stream payments

This pallet supports creating *streams* – ongoing payments. Once a stream is opened,
on every block a specified amount of funds will be transferred from the creator's account
to the given target account until the stream is closed.

## Interface
TBD 

## TODO:

### Maintenance

- [ ] Fill in README
- [ ] Add docstrings
- [ ] Compute proper weights
- [x] Write some tests

### Features

- [ ] Auto-close streams when account runs out of funds (?)
- [ ] Add an optional total spend limit for stream and reserve funds for limited streams.
- [ ] Disallow origin == target (that's most probably user's mistake).

License: Apache-2.0
