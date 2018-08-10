[![Build Status](https://travis-ci.com/noahp/http-status-print.svg?branch=master)](https://travis-ci.com/noahp/http-status-print)
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
## Edge
```bash
cargo install --git https://github.com/noahp/http-status-print
```
