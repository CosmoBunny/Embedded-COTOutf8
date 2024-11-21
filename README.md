# Embedded-COTOutf8

[![crates.io](https://img.shields.io/crates/v/embedded_cotoutf8)](https://crates.io/crates/embedded_cotoutf8)

![LOGO](https://raw.githubusercontent.com/unknownK19/Embedded-COTOutf8/88ca46a2b651e36d1d0443971bee8efec4134a55/Logo.svg)

COTO is derived from the Gujarati word કોતો, meaning 'engrave.' It refers to a concept where data sizes are fixed. COTO is a library that translates numerical primitives into readable ASCII UTF-8 arrays.

## Example󰙨

Convert primitives data type to specific Byte(character) array for str

```rust
    // for i8
    let num: i8 = -127;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    assert_eq!(result, "-127")
    // for f32
    let num: f32 = 1524.001;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result); // OUTPUT:` 1524.024`

```

Debug or Display for ufmt _By default ufmt feature enabled_ or fmt

```rust
    let num = DebugODisplay(44245.12f32);
    println!("{}", num);
    ufmt::uwriteln!(serial, "{}", num); // for ufmt
```

implemented for `i8`,`i16`,`132`,`i64`,`u8`,`u16`,`u32`,`f32`,`f64`
