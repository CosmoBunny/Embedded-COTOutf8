#![no_std]

use core::{
    f64,
    fmt::{Debug, Display},
};

#[cfg(feature = "floato")]
pub mod floato;

pub trait COtoUTF8<const O: usize> {
    fn coto_utf8(&self) -> [u8; O];
}

pub trait COtoHex<const O: usize> {
    fn coto_hex(&self) -> [u8; O];
}

impl COtoUTF8<3> for u8 {
    fn coto_utf8(&self) -> [u8; 3] {
        let mut digit: [u8; 3] = [0; 3];
        let n = *self;

        for (i, j) in digit.iter_mut().enumerate() {
            *j = b'0' + n / (10u8.pow(i as u32)) % 10
        }

        digit.reverse();
        digit
    }
}

pub struct DebugODisplay<T: COtoUTF8<O>, const O: usize>(pub T);

impl<T, const O: usize> Debug for DebugODisplay<T, O>
where
    T: COtoUTF8<O>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(&self.0.coto_utf8()).unwrap())
            .unwrap();
        Ok(())
    }
}

impl<T, const O: usize> Display for DebugODisplay<T, O>
where
    T: COtoUTF8<O>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::str::from_utf8(&self.0.coto_utf8()).unwrap())
            .unwrap();
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

impl COtoUTF8<4> for i8 {
    fn coto_utf8(&self) -> [u8; 4] {
        let mut digit: [u8; 4] = [0; 4];
        digit[0] = if self.is_negative() { b'-' } else { b' ' };
        let n = self.abs();

        for (i, j) in digit[1..].iter_mut().enumerate() {
            *j = (n / (10i8.pow(i as u32)) % 10) as u8 + b'0'
        }

        digit[1..].reverse();

        digit
    }
}

impl COtoUTF8<6> for i16 {
    fn coto_utf8(&self) -> [u8; 6] {
        let mut digit: [u8; 6] = [0; 6];
        digit[0] = if self.is_negative() { b'-' } else { b' ' };
        let n = self.abs();

        for (i, j) in digit[1..].iter_mut().enumerate() {
            *j = (n / (10i16.pow(i as u32)) % 10) as u8 + b'0'
        }

        digit[1..].reverse();

        digit
    }
}

impl COtoUTF8<11> for i32 {
    fn coto_utf8(&self) -> [u8; 11] {
        let mut digit: [u8; 11] = [0; 11];
        digit[0] = if self.is_negative() { b'-' } else { b' ' };
        let n = self.abs();

        for (i, j) in digit[1..].iter_mut().enumerate() {
            *j = (n / (10i32.pow(i as u32)) % 10) as u8 + b'0'
        }

        digit[1..].reverse();

        digit
    }
}
impl COtoUTF8<20> for i64 {
    fn coto_utf8(&self) -> [u8; 20] {
        let mut digit = [0u8; 20];
        digit[0] = if self.is_negative() { b'-' } else { b' ' };
        let n = self.abs();

        for (i, j) in digit[1..].iter_mut().enumerate() {
            *j = (n / (10i64.pow(i as u32)) % 10) as u8 + b'0'
        }

        digit[1..].reverse();

        digit
    }
}

impl COtoUTF8<10> for u32 {
    fn coto_utf8(&self) -> [u8; 10] {
        let mut digit: [u8; 10] = [0; 10];
        let n = *self;

        for (i, j) in digit.iter_mut().enumerate() {
            *j = (n / (10u32.pow(i as u32)) % 10) as u8 + b'0'
        }

        digit.reverse();
        digit
    }
}

impl COtoUTF8<5> for u16 {
    fn coto_utf8(&self) -> [u8; 5] {
        let mut digit: [u8; 5] = [0; 5];
        let n = *self;

        for (i, j) in digit.iter_mut().enumerate() {
            *j = (n / (10u16.pow(i as u32)) % 10) as u8 + b'0'
        }

        digit.reverse();
        digit
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
    assert_eq!(" 123", core::str::from_utf8(&123i8.coto_utf8()).unwrap());
    assert_eq!(
        "-12345",
        core::str::from_utf8(&(-12345i16).coto_utf8()).unwrap()
    );
    assert_eq!(
        " 1234567890",
        core::str::from_utf8(&1234567890i32.coto_utf8()).unwrap()
    );
    assert_eq!(
        " 1.234567",
        core::str::from_utf8(&1.234567f32.coto_utf8()).unwrap()
    )
}
