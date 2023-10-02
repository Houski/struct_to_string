# struct_to_string

A Rust procedural macro crate for converting struct definitions into a string representation.

Useful for things like API documentation where you want to display your Rust structs on a webpage.

To further assist with applications such as API documentation, it can also convert your structs to structs/classes in other languages, such as Go, Python, TypeScript, Java, and C#.

The conversion of structs to other languages may not always be perfect for complicated structs.

## Installation

Add `struct_to_string` to your `Cargo.toml`:

```toml
[dependencies]
struct_to_string = "0.2.0"
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

You can then use the generated `to_rust_string()` function:

```rust
let my_struct_as_string = MyStruct::to_rust_string();
```

Which outputs a string like:

```rust
"struct MyStruct {
    field1: i32,
    field2: String,
}"
```

You can also convert your structs to structs/class strings in other languages (useful for API documentation):

```rust
let my_struct_as_typescript_string = MyStruct::to_typescript_string();
```

Which outputs a string like:

```rust
"interface MyStruct {
  field1: number;
  field2: string;
}"
```

The languages that this crate can convert Rust structs can be converted to are:

- Rust
- Go
- Python
- TypeScript
- Java
- C#

Though conversion may not always be perfect for complicated structs.

## License

```
MIT
```
