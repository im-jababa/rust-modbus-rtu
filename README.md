# modbus-rtu

[![Github](https://img.shields.io/badge/github-im--jababa%2Frust--modbus--rtu-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/im-jababa/rust-modbus-rtu)
[![Crates.io](https://img.shields.io/badge/crates.io-modbus--rtu-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/modbus-rtu)
[![Docs.rs](https://img.shields.io/badge/docs.rs-modbus--rtu-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/modbus-rtu)
[![Build](https://img.shields.io/github/actions/workflow/status/im-jababa/rust-modbus-rtu/rust.yml?branch=main&style=for-the-badge)](https://github.com/im-jababa/rust-modbus-rtu/actions?query=branch%3Amain)

This crate provides helpers for building and decoding standard Modbus RTU request and response packets.

---

# Usage

## Sending

First, construct the function you want to issue.
The following example reads four input registers starting at address `0x1234`.

```rust
use modbus_rtu::Function;

let starting_address: u16 = 0x1234;
let quantity: usize = 4;
let function = Function::ReadInputRegisters { starting_address, quantity };
```

Next, build the request with the target device information and timeout.

```rust
use modbus_rtu::{Function, Request};

...

let modbus_id: u8 = 1;
let timeout: std::time::Duration = std::time::Duration::from_millis(100);
let request = Request::new(1, &function, timeout);
```

Finally, convert the request into a Modbus RTU frame.

```rust
...

let packet: Box<[u8]> = request.to_bytes().expect("Failed to build request packet");
```

*Sending the bytes to an actual device is not yet implemented in this crate.*

---

## Receiving

*Receiving bytes from a physical device is also outside the scope of this crate.*
*This example assumes you already obtained the raw response bytes.*

With the original request available, attempt to decode the response bytes as shown below.

```rust
use modbus_rtu::Response;

...

let bytes: &[u8] = ... ; // user-implemented receive logic

let response = Response::from_bytes(&request, bytes).expect("Failed to analyze response packet");

match response {
    Response::Value(value) => {
        let _ = value[0];   // value at address 0x1234
        let _ = value[1];   // value at address 0x1235
        let _ = value[2];   // value at address 0x1236
        let _ = value[3];   // value at address 0x1237
    },
    Response::Exception(e) => {
        eprintln!("device responded with exception: {e}");
    },
    _ => unreachable!(),
}
```
