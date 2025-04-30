# War3StatsObserver 

Rust bindings for the Warcraft 3 Stats Observer API memory map.
Great for a streaming or build order overlay.

Top level binding to access everything is `ObserverData` struct.
```rust
let od = match ObserverData::new() {
    Ok(od) => od,
    Err(e) => {
        eprintln!("Error opening observer API. Is Warcraft3 running? Error: {e:?}");
        // handle error
    }
};
```

A refresh rate of 0 (measured in milliseconds) disables the API.
Default rate is 500ms when using `new()`, but you can change it.

The structs are packed due to how the memory map is laid out.
This makes it a pain to work with certain fields of the structs at times because rust does not allow a reference to an unaligned field.
To get around this, you can make a copy in place by wrapping a field in `{}`
```rust
println!("version: {}", { od.version });
```

See the examples folder for more.