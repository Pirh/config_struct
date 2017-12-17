static_conf
===

This is a library for converting config files into matching source files at build time.

Usage
---

Use this library in a `build.rs` file:

```rust
extern crate static_conf;

fn main() {
    static_conf::construct_config("config.toml", "src/config.rs");
}
```

This will take the following config file:

```toml
name = "Config name"
```

... and generate the following source file:

```rust
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub name: Cow<'static, str>
}

pub const CONFIG: Config = Config {
    name: Cow::Borrowed("Config name")
};
```

TODO
---

### For sure
-   Work with any input data (currently only handling strings)
-   Allow specifying which traits to derive
-   Allow nested objects
-   Decent error handling
-   Allow naming the root struct

### Eventually
-   Work with yaml, json, etc.
-   Features to opt in/out of the generated const
-   Allow generating the const only in release mode

### Like, mayyybe
-   Possibly allow specifying imports, and custom types
-   Try to avoid generating the same struct twice
-   Use fixed-size arrays instead of slices for some (configurable) size threshold

