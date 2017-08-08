# debug &emsp; [![Build Status]][travis] [![Latest Version]][crates.io] [![docs]][docs.rs] [![License]][License Link]

[Build Status]: https://api.travis-ci.org/zaaack/debug-rs.svg?branch=master
[travis]: https://travis-ci.org/zaaack/debug-rs
[Latest Version]: https://img.shields.io/crates/v/debug.svg
[crates.io]: https://crates.io/crates/debug
[docs]: https://docs.rs/debug/badge.svg
[docs.rs]: https://docs.rs/debug/
[License]: https://img.shields.io/badge/license-MIT-blue.svg
[License Link]: https://github.com/zaaack/debug-rs/blob/master/LICENSE

A debug crate for rust inspired by NodeJS [debug](https://github.com/visionmedia/debug) module.


## Usage
Here is the code in examples folder:  
```rust
#[macro_use]
extern crate debug;


fn main() {
    debug!(666, 33, "aaa");

    debug!(vec![1, 2, 3]);
}
```

Then run with environment variable `DEBUG=debug` (because the examples' package name is still `debug`):

![](docs/out.png)

If environment variable `DEBUG` is undefined, then it won't print anything.


## DEBUG format

1. Single glob pattern for `<package name>:<file name>`: e.g. `DEBUG=debug*`
2. Multi glob patterns separated by comma: e.g. `DEBUG=debug:examples*,hyper*,`


## Output format

```
<package name>:<file name>:L<line number> ...custom variables
```

## License

MIT
