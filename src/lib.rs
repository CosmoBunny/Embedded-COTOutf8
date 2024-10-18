#![no_std]

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
            *j = b"0"[0] + n / (10u8.pow(i as u32)) % 10
        }

        digit.reverse();
        digit
    }
}
/**
F32 (cap F 32) is another floating point in future. Unlike f32 IEEE 754-2008, it has less error but no precise as f32.
so here's How it look

## F32

- exponential of 10 : E,

- Mantissa : M

EEEE_EEEE MMMM_MMMM_MMMM_MMMM_MMMM_MMMM
|---i8--| |--- i24 = i8 + u16---------|

NOTE: Current WIP
*/

#[derive(Default)]
pub struct F32(i8, i8, u16);

impl F32 {
    pub fn mantissa(&self) -> i32 {
        let mut buffer = [0u8; 4];
        buffer[1..=1].copy_from_slice(&self.1.to_be_bytes());
        buffer[2..=3].copy_from_slice(&self.2.to_be_bytes());
        if self.1.is_negative() {
            buffer[0] = 0xFF;
            buffer[2..=3].copy_from_slice(&(!self.2).to_be_bytes());
        };
        i32::from_be_bytes(buffer)
    }
    pub fn exponent(&self) -> i8 {
        self.0
    }
}

impl From<u8> for F32 {
    fn from(value: u8) -> Self {
        Self(0, 0, value as u16)
    }
}

impl From<i8> for F32 {
    fn from(value: i8) -> Self {
        F32(
            0,
            i8::from_be_bytes([if value.ge(&0) { 0x00 } else { 0xff }]),
            if value.ge(&0) {
                u16::from_be_bytes([0x00, value.to_be_bytes()[0]])
            } else {
                !u16::from_be_bytes([0xff, value.to_be_bytes()[0]])
            },
        )
    }
}

impl From<u16> for F32 {
    fn from(value: u16) -> Self {
        Self(0, 0, value)
    }
}
impl From<i16> for F32 {
    fn from(value: i16) -> Self {
        let numbytes = value.to_be_bytes();
        F32(
            0,
            i8::from_be_bytes([if value.ge(&0) { 0x00 } else { 0xff }]),
            if value.ge(&0) {
                u16::from_be_bytes([numbytes[0], numbytes[1]])
            } else {
                !u16::from_be_bytes([numbytes[0], numbytes[1]])
            },
        )
    }
}
impl From<u32> for F32 {
    fn from(value: u32) -> Self {
        let mut num = value;
        let mut exponential: i8 = 0;
        while !(0..={ 2u32.pow(24) - 1 }).contains(&num) {
            exponential += 1;
            num /= 10;
        }
        let numbytes = num.to_be_bytes();
        F32(
            exponential,
            numbytes[1] as i8,
            u16::from_be_bytes([numbytes[2], numbytes[3]]),
        )
    }
}
impl From<i32> for F32 {
    fn from(value: i32) -> Self {
        let mut num = value;
        let mut exponential: i8 = 0;
        while !(-{ 2i32.pow(23) }..={ 2i32.pow(23) - 1 }).contains(&num) {
            if num.is_positive() {
                exponential += 1
            } else {
                exponential -= 1
            }
            num /= 10;
        }
        let numbytes = num.to_be_bytes();
        F32(
            exponential,
            numbytes[1] as i8,
            if num.ge(&0) {
                u16::from_be_bytes([numbytes[2], numbytes[3]])
            } else {
                !u16::from_be_bytes([numbytes[2], numbytes[3]])
            },
        )
    }
}
impl From<f32> for F32 {
    fn from(value: f32) -> Self {
        let mut num = if value < 0. { -value } else { value };
        let mut exponential: i8 = 0;
        while !(0.0..={ 2i32.pow(23) - 1 } as f32).contains(&num) {
            if num.is_sign_positive() {
                exponential += 1;
                num /= 10.
            } else {
                exponential -= 1;
                num *= 10.
            }
        }
        let numbytes = (num as i32).to_be_bytes();
        F32(
            exponential,
            numbytes[1] as i8,
            if num.ge(&0.) {
                u16::from_be_bytes([numbytes[2], numbytes[3]])
            } else {
                !u16::from_be_bytes([numbytes[2], numbytes[3]])
            },
        )
    }
}
// TODO:
// impl Add for F32 {
//     type Output = F32;
//     fn add(self, rhs: Self) -> Self::Output {
//
//     }
// }

impl COtoHex<2> for u8 {
    fn coto_hex(&self) -> [u8; 2] {
        let mut sample = [0u8; 2];

        for (i, j) in sample.iter_mut().enumerate() {
            *j = match (self >> (4 * i) & 0x0F) % 16 {
                int @ 0..=9 => b"0"[0] + int,
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
                int @ 0..=9 => b"0"[0] + int,
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
                int @ 0..=9 => b"0"[0] + int,
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
        buffer[0] = if self.is_sign_negative() {
            b"-"[0]
        } else {
            b" "[0]
        };
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
                buffer[2 + exponent] = b"."[0];
                let cotonum = ((self * 1_000_000.0) as i32).coto_utf8();
                buffer[1..={ exponent + 1 }].copy_from_slice(&cotonum[{ 4 - exponent }..=4]);
                buffer[{ 3 + exponent }..].copy_from_slice(&cotonum[{ 5 + exponent }..]);
            } else {
                buffer[5..].copy_from_slice(&exponential.coto_utf8());
                buffer[4] = b"E"[0];
                buffer[1..=3].copy_from_slice(&num.coto_utf8()[1..=3])
            }
        }
        buffer
    }
}

impl COtoUTF8<4> for i8 {
    fn coto_utf8(&self) -> [u8; 4] {
        let mut digit: [u8; 4] = [0; 4];
        digit[0] = if self.is_negative() { b"-"[0] } else { b" "[0] };
        let n = self.abs();

        for (i, j) in digit[1..].iter_mut().enumerate() {
            *j = (n / (10i8.pow(i as u32)) % 10) as u8 + b"0"[0]
        }

        digit[1..].reverse();

        digit
    }
}

impl COtoUTF8<6> for i16 {
    fn coto_utf8(&self) -> [u8; 6] {
        let mut digit: [u8; 6] = [0; 6];
        digit[0] = if self.is_negative() { b"-"[0] } else { b" "[0] };
        let n = self.abs();

        for (i, j) in digit[1..].iter_mut().enumerate() {
            *j = (n / (10i16.pow(i as u32)) % 10) as u8 + b"0"[0]
        }

        digit[1..].reverse();

        digit
    }
}

impl COtoUTF8<11> for i32 {
    fn coto_utf8(&self) -> [u8; 11] {
        let mut digit: [u8; 11] = [0; 11];
        digit[0] = if self.is_negative() { b"-"[0] } else { b" "[0] };
        let n = self.abs();

        for (i, j) in digit[1..].iter_mut().enumerate() {
            *j = (n / (10i32.pow(i as u32)) % 10) as u8 + b"0"[0]
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
            *j = (n / (10u32.pow(i as u32)) % 10) as u8 + b"0"[0]
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
            *j = (n / (10u16.pow(i as u32)) % 10) as u8 + b"0"[0]
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
