# About this crate

This crate offers convenience macros for [gvdb](https://crates.io/crates/gvdb).
The macros are `include_gresource_from_xml!()` and `include_gresource_from_dir!()`

## Examples

Compile a GResource XML file and include the bytes in the file.

```rust
use gvdb_macros::include_gresource_from_xml;
const GRESOURCE_BYTES: &[u8] = include_gresource_from_xml!("test/test3.gresource.xml");
```

Scan a directory and create a GResource file with all the contents of the directory.

```rust
use gvdb_macros::include_gresource_from_dir;
const GRESOURCE_BYTES: &[u8] = include_gresource_from_dir!("/gvdb/rs/test", "test/");
```
