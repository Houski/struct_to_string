# struct_to_string

A Rust procedural macro crate for converting struct definitions into a string representation.

Useful for things like API documentation where you want to display your Rust structs on a webpage.

## Installation

Add `struct_to_string` to your `Cargo.toml`:

```toml
[dependencies]
struct_to_string = "0.1.0"
```

## Usage

Add the `#[derive(StructToString)]` attribute to the structs you'd like to generate string representations for:

```rust
#[derive(StructToString)]
struct MyStruct {
    field1: i32,
    field2: String,
}
```

You can then use the generated `to_string()` function:

```rust
let my_struct_as_string = MyStruct::to_string();
```

## Example Output

```text
struct MyStruct {
    field1: i32,
    field2: String,
}
```

## License

MIT
