# <div align="center">log-attributes</div>

<div align="center">

[![crates.io](https://img.shields.io/crates/v/log-attributes?style=flat-square)](https://crates.io/crates/log-attributes)
[![docs.rs](https://img.shields.io/docsrs/log-attributes?style=flat-square)](https://docs.rs/log-attributes)
[![licence](https://img.shields.io/crates/l/log-attributes?style=flat-square)](https://github.com/DzenanJupic/log-attributes)

![build](https://img.shields.io/github/workflow/status/DzenanJupic/log-attributes/Check%20and%20Build%20code?style=flat-square)

</div>

A set of function log attributes.

*This project is currently in active development. It's already usable, but not nearly feature complete.*

## Overview

`log-attributes` provides a set of attribute macros, that allow to easily log a function's inputs and return value.

## `1.0` Roadmap

- [x] log after function return
- [ ] log before function call
- [x] allow

## Examples

```rust
use log_attributes::{log, info};

#[log(warn, "{fn} was called with {a} and returned {return}")]
fn using_log_attribute(a: &[u32]) -> u32 {
    a[0]
}

#[info("{fn} was called")]
fn using_level_attribute() {
    // -- snip --
}
```

Stay tuned for more.

## Contributing

log-attributes is still in a pretty early stage, and you are welcome to contribute to it! The goal is to make logging
function inputs and returns more ergonomically.

This project is 100% open source. Any contribution submitted for inclusion in rustube by you, shall have both the MIT
licence and the Apache-2.0 licence, and shall be licensed as MIT OR Apache-2.0, without any additional terms or
conditions.

## Licence

This project is licensed under the terms of the MIT licence or the Apache-2.0 licence, at your own choice.
