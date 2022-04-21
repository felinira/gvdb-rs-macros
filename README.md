# About this crate

This crate offers convenience macros for [gvdb](https://crates.io/crates/gvdb).
Currently the only implemented macro is `include_gresource_from_xml!()`

## Example

This example compiles a GResource XML file and includes the bytes in the file.

```rust
use gvdb_macros::include_gresource_from_xml;
const GRESOURCE_BYTES: &[u8] = include_gresource_from_xml!("test/test3.gresource.xml");
```
