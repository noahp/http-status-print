[![](https://img.shields.io/badge/GitHub-noahp/http--status--print-8da0cb?style=flat-square&logo=github)](https://github.com/noahp/http-status-print)
[![](https://img.shields.io/github/actions/workflow/status/noahp/http-status-print/ci.yml?style=flat-square&branch=main)](https://github.com/noahp/http-status-print/actions?query=branch%3Amain+)
[![](https://img.shields.io/github/license/noahp/http-status-print?color=blue&style=flat-square)](https://github.com/noahp/http-status-print)
[![Crates.io - http-status-print](https://img.shields.io/crates/v/http-status-print.svg?style=flat-square&maxAge=2592000)](https://crates.io/crates/http-status-print)

# http-status-print

Utility to print short descriptions for HTTP error codes:

```bash
➜ http-status-print 123 200 501
123 - 123 <unknown status code>
200 - OK : https://httpstatuses.com/200
501 - Not Implemented : https://httpstatuses.com/501
```

Try to learn a little rust while building this tool.

# Install

Installation is via the cargo package manager. You'll need to [rustup](https://www.rustup.rs/) if you don't have this already!

## Release

```bash
cargo install http-status-print
```

## Development Version

```bash
cargo install --git https://github.com/noahp/http-status-print
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
