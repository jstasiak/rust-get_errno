get_errno
=========

> A way to extract ``errno`` from ``std::io::Error``

## Example

```rust
use std::io::Error;
use get_errno::get_errno;

println!("{}", get_errno(&Error::from_os_error(47)));
// Some(47)

println!("{}", get_errno(&Error::new(ErrorKind::Other, "description", None)));
// None
```

## Status

Experimental

## License

MIT
