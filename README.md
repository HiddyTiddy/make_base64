# `make_base64`

base64-encode a buffer.

## usage

```rust
use make_base64::base64;

let input = "hello rust!\n";
let mut buffer = [0; 128];
let read = base64(input, &mut buffer).expect("enough space");
assert_eq!(
    &buffer[..read],
    "aGVsbG8gcnVzdCEK"
        .as_bytes()
);
```
