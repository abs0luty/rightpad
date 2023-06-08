<p align="center">
<img width="30%" src="./logo.png" />
</p>

# `rightpad-str` crate.

Obviously the second best crate (after `leftpad-str`) for the Rust programming language. The crate serves the only goal of implementing `rightpad` function:

```rs
pub fn rightpad(input: &str, width: usize, padding_char: char) -> String;
```

# Examples

```rs
assert_eq!(rightpad("hello", 8, '*'), "hello***");
assert_eq!(rightpad("rust", 6, ' '), "rust  ");
```
