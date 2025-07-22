#![no_std]

/**
COTO is derived from the Gujarati word કોતો, meaning 'engrave.' It refers to a concept where data sizes are fixed. COTO is a library that translates numerical primitives into readable ASCII UTF-8 arrays.

## Example󰙨

Convert primitives data type to specific Byte(character) array for str

```rust
    // for i8
    let num: i8 = -127;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    assert_eq!(result, "-127");
    // for f32
    let num: f32 = 1524.001;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result); // OUTPUT:` 1524.024`

```

Debug or Display for ufmt _By default ufmt feature enabled_ or fmt

```rust
    use embedded_cotoutf8::DebugODisplay;

    let num = DebugODisplay(44245.12f32);
    println!("{}", num);
    ufmt::uwriteln!(serial, "{}", num); // for ufmt
```

implemented for `i8`,`i16`,`132`,`i64`,`u8`,`u16`,`u32`,`f32`,`f64`
*/
use core::{
    f64,
    fmt::{Debug, Display},
};

use ufmt::{uDebug, uDisplay};

pub trait COtoUTF8<const O: usize> {
    fn coto_utf8(&self) -> [u8; O];
}

// u8 - max value 255 (3 digits)
impl COtoUTF8<3> for u8 {
    fn coto_utf8(&self) -> [u8; 3] {
        let mut result = [0u8; 3];
        let mut n = *self;

        if n == 0 {
            result[2] = b'0';
            return result;
        }

        let mut pos = 2;
        while n > 0 {
            result[pos] = (n % 10) + b'0';
            n /= 10;
            pos = pos.saturating_sub(1);
        }

        result
    }
}

// u16 - max value 65535 (5 digits)
impl COtoUTF8<5> for u16 {
    fn coto_utf8(&self) -> [u8; 5] {
        let mut result = [0u8; 5];
        let mut n = *self;

        if n == 0 {
            result[4] = b'0';
            return result;
        }

        let mut pos = 4;
        while n > 0 {
            result[pos] = (n % 10) as u8 + b'0';
            n /= 10;
            pos = pos.saturating_sub(1);
        }

        result
    }
}

// u32 - max value 4294967295 (10 digits)
impl COtoUTF8<10> for u32 {
    fn coto_utf8(&self) -> [u8; 10] {
        let mut result = [0u8; 10];
        let mut n = *self;

        if n == 0 {
            result[9] = b'0';
            return result;
        }

        let mut pos = 9;
        while n > 0 {
            result[pos] = (n % 10) as u8 + b'0';
            n /= 10;
            pos = pos.saturating_sub(1);
        }

        result
    }
}

// u64 - max value 18446744073709551615 (20 digits)
impl COtoUTF8<20> for u64 {
    fn coto_utf8(&self) -> [u8; 20] {
        let mut result = [0u8; 20];
        let mut n = *self;

        if n == 0 {
            result[19] = b'0';
            return result;
        }

        let mut pos = 19;
        while n > 0 {
            result[pos] = (n % 10) as u8 + b'0';
            n /= 10;
            pos = pos.saturating_sub(1);
        }

        result
    }
}

// i8 - range -128 to 127 (4 digits including sign)
impl COtoUTF8<4> for i8 {
    fn coto_utf8(&self) -> [u8; 4] {
        let mut result = [0u8; 4];
        let mut n = self.abs() as u8;

        if *self == 0 {
            result[3] = b'0';
            return result;
        }

        let mut pos = 3;
        while n > 0 {
            result[pos] = (n % 10) + b'0';
            n /= 10;
            pos = pos.saturating_sub(1);
        }

        if *self < 0 {
            result[pos] = b'-';
        }

        result
    }
}

// i16 - range -32768 to 32767 (6 digits including sign)
impl COtoUTF8<6> for i16 {
    fn coto_utf8(&self) -> [u8; 6] {
        let mut result = [0u8; 6];
        let mut n = self.abs() as u16;

        if *self == 0 {
            result[5] = b'0';
            return result;
        }

        let mut pos = 5;
        while n > 0 {
            result[pos] = (n % 10) as u8 + b'0';
            n /= 10;
            pos = pos.saturating_sub(1);
        }

        if *self < 0 {
            result[pos] = b'-';
        }

        result
    }
}

// i32 - range -2147483648 to 2147483647 (11 digits including sign)
impl COtoUTF8<11> for i32 {
    fn coto_utf8(&self) -> [u8; 11] {
        let mut result = [0u8; 11];
        let mut n = self.abs() as u32;

        if *self == 0 {
            result[10] = b'0';
            return result;
        }

        let mut pos = 10;
        while n > 0 {
            result[pos] = (n % 10) as u8 + b'0';
            n /= 10;
            pos = pos.saturating_sub(1);
        }

        if *self < 0 {
            result[pos] = b'-';
        }

        result
    }
}

// i64 - range -9223372036854775808 to 9223372036854775807 (20 digits including sign)
impl COtoUTF8<20> for i64 {
    fn coto_utf8(&self) -> [u8; 20] {
        let mut result = [0u8; 20];
        let mut n = self.abs() as u64; // Use u64 to handle i64::MIN safely

        if *self == 0 {
            result[19] = b'0';
            return result;
        }

        let mut pos = 19;
        while n > 0 {
            result[pos] = (n % 10) as u8 + b'0';
            n /= 10;
            pos = pos.saturating_sub(1);
        }

        if *self < 0 {
            result[pos] = b'-';
        }

        result
    }
}

pub trait COtoHex<const O: usize> {
    fn coto_hex(&self) -> [u8; O];
}

pub struct DebugODisplay<T: COtoUTF8<O>, const O: usize>(pub T);

impl<T, const O: usize> Debug for DebugODisplay<T, O>
where
    T: COtoUTF8<O>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(&self.0.coto_utf8()).unwrap())?;
        Ok(())
    }
}

impl<T, const O: usize> Display for DebugODisplay<T, O>
where
    T: COtoUTF8<O>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(&self.0.coto_utf8()).unwrap())?;
        Ok(())
    }
}

#[cfg(feature = "ufmt")]
impl<T, const O: usize> uDebug for DebugODisplay<T, O>
where
    T: COtoUTF8<O>,
{
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_str(core::str::from_utf8(&self.0.coto_utf8()).unwrap())?;
        Ok(())
    }
}

#[cfg(feature = "ufmt")]
impl<T, const O: usize> uDisplay for DebugODisplay<T, O>
where
    T: COtoUTF8<O>,
{
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_str(core::str::from_utf8(&self.0.coto_utf8()).unwrap())?;
        Ok(())
    }
}

impl COtoHex<2> for u8 {
    fn coto_hex(&self) -> [u8; 2] {
        let mut sample = [0u8; 2];

        for (i, j) in sample.iter_mut().enumerate() {
            *j = match (self >> (4 * i) & 0x0F) % 16 {
                int @ 0..=9 => b'0' + int,
                alpha @ 10..=15 => 55 + alpha,
                _ => 0,
            }
        }
        sample.reverse();
        sample
    }
}

impl COtoHex<4> for u16 {
    fn coto_hex(&self) -> [u8; 4] {
        let mut sample = [0u8; 4];

        for (i, j) in sample.iter_mut().enumerate() {
            *j = match ((self >> (4 * i) & 0x0F) % 16) as u8 {
                int @ 0..=9 => b'0' + int,
                alpha @ 10..=15 => 55 + alpha,
                _ => 0,
            }
        }
        sample.reverse();
        sample
    }
}

impl COtoHex<8> for u32 {
    fn coto_hex(&self) -> [u8; 8] {
        let mut sample = [0u8; 8];

        for (i, j) in sample.iter_mut().enumerate() {
            *j = match ((self >> (4 * i) & 0x0F) % 16) as u8 {
                int @ 0..=9 => b'0' + int,
                alpha @ 10..=15 => 55 + alpha,
                _ => 0,
            }
        }
        sample.reverse();
        sample
    }
}

impl COtoUTF8<9> for f32 {
    fn coto_utf8(&self) -> [u8; 9] {
        let mut buffer = [0u8; 9];
        buffer[0] = if self.is_sign_negative() { b'-' } else { b' ' };
        if self.is_infinite() {
            buffer[1..].copy_from_slice(b"INFINITY");
        } else {
            let mut num = if *self < 0. { -*self } else { *self };
            let mut exponential: i8 = 0;
            while !(0. ..=10.).contains(&num) {
                if num < 0. {
                    exponential -= 1;
                    num *= 10.;
                } else {
                    exponential += 1;
                    num /= 10.;
                }
            }
            if (-6..=3).contains(&exponential) {
                let exponent = exponential.max(0) as usize;
                buffer[2 + exponent] = b'.';
                let cotonum = ((self * 1_000_000.0) as i32).coto_utf8();
                buffer[1..={ exponent + 1 }].copy_from_slice(&cotonum[{ 4 - exponent }..=4]);
                buffer[{ 3 + exponent }..].copy_from_slice(&cotonum[{ 5 + exponent }..]);
            } else {
                buffer[6] = exponential.coto_utf8()[0].max(b'+');
                buffer[7..].copy_from_slice(&exponential.coto_utf8()[2..]);
                buffer[5] = b'E';
                buffer[1..=3].copy_from_slice(&num.coto_utf8()[1..=3])
            }
        }
        buffer
    }
}

impl COtoUTF8<11> for f64 {
    fn coto_utf8(&self) -> [u8; 11] {
        let mut buffer = [0u8; 11];
        buffer[0] = if self.is_sign_negative() { b'-' } else { b' ' };
        if self.is_infinite() {
            buffer[1..].copy_from_slice(b"INFINITY");
        } else {
            let mut num = if *self < 0. { -*self } else { *self };
            let mut exponential: i16 = 0;
            while !(0. ..=10.).contains(&num) {
                if num < 0. {
                    exponential -= 1;
                    num *= 10.;
                } else {
                    exponential += 1;
                    num /= 10.;
                }
            }
            if (-8..=5).contains(&exponential) {
                let exponent = exponential.max(0) as usize;
                buffer[2 + exponent] = b'.';
                let cotonum = ((self * 100_000_000.0) as i64).coto_utf8();
                buffer[1..={ exponent + 1 }].copy_from_slice(&cotonum[{ 11 - exponent }..=11]);
                buffer[{ 3 + exponent }..].copy_from_slice(&cotonum[{ 12 + exponent }..]);
            } else {
                buffer[7] = exponential.coto_utf8()[0].max(b'+');
                buffer[8..].copy_from_slice(&exponential.coto_utf8()[3..]);
                buffer[6] = b'E';
                buffer[1..=5].copy_from_slice(&num.coto_utf8()[1..=5])
            }
        }
        buffer
    }
}

#[test]
fn utf8test() {
    assert_eq!("123", core::str::from_utf8(&123u8.coto_utf8()).unwrap());
    assert_eq!(
        "12345",
        core::str::from_utf8(&12345u16.coto_utf8()).unwrap()
    );
    assert_eq!(
        "1234567890",
        core::str::from_utf8(&1234567890u32.coto_utf8()).unwrap()
    );
    assert_eq!("\0123", core::str::from_utf8(&123i8.coto_utf8()).unwrap());
    assert_eq!(
        "-12345",
        core::str::from_utf8(&(-12345i16).coto_utf8()).unwrap()
    );
    assert_eq!(
        "\01234567890",
        core::str::from_utf8(&1234567890i32.coto_utf8()).unwrap()
    );
    assert_eq!(
        " 1.234567",
        core::str::from_utf8(&1.234567f32.coto_utf8()).unwrap()
    )
}
