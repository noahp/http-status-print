[![Crates.io - http-status-print](https://img.shields.io/crates/v/http-status-print.svg?style=flat-square)](https://crates.io/crates/http-status-print)
# http-status-print
Utility to print short descriptions for HTTP error codes:
```bash
➜ http-status-print 234 503
234 <unknown status code> : https://httpstatuses.com/234
503 Service Unavailable : https://httpstatuses.com/503
```
Try to learn a little rust while building this tool.

# Install
```bash
cargo install --git https://github.com/noahp/http-status-print
```
