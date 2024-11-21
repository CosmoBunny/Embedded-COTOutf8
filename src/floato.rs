/**
F32 (cap F 32) is another way of floating point which doesn't depend on precision except correctness like 0.1 + 0.2 will equal 0.3.
Unlike f32 IEEE 754-2008, it has less error but no precise as f32.
so here's How it look

## F32

- exponential of 10 : E,

- Mantissa : M

| EEEE_EEEE | MMMM_MMMM_MMMM_MMMM_MMMM_MMMM_MMMM_MMMM |
|-----------|-----------------------------------------|
| i8        | i32                                     |

NOTE: Currently WIP
*/
#[derive(Default)]
pub struct F32(i8, i32);

impl F32 {
    pub fn mantissa(&self) -> i32 {
        self.1
    }
    pub fn exponent(&self) -> i8 {
        self.0
    }
}

impl From<u8> for F32 {
    fn from(value: u8) -> Self {
        Self(0, value as i32)
    }
}

impl From<i8> for F32 {
    fn from(value: i8) -> Self {
        F32(0, value as i32)
    }
}

impl From<u16> for F32 {
    fn from(value: u16) -> Self {
        Self(0, value as i32)
    }
}
impl From<i16> for F32 {
    fn from(value: i16) -> Self {
        F32(0, value as i32)
    }
}
impl From<u32> for F32 {
    fn from(value: u32) -> Self {
        let mut num = value;
        let mut exponential: i8 = 0;
        while !(0..={ i32::MAX as u32 }).contains(&num) {
            exponential += 1;
            num /= 10;
        }
        F32(exponential, num as i32)
    }
}
impl From<i32> for F32 {
    fn from(value: i32) -> Self {
        F32(0, value)
    }
}
impl From<f32> for F32 {
    fn from(value: f32) -> Self {
        let (mut num, neg) = if value < 0. {
            (-value, true)
        } else {
            (value, false)
        };
        let mut exponential: i8 = 0;

        if value.is_infinite() {
            if value.is_sign_negative() {
                return F32(i8::MAX, i32::MIN);
            } else {
                return F32(i8::MAX, i32::MAX);
            }
        } else if value == 0.0 {
            return F32(0, 0);
        } else {
            while !(0. ..=i32::MAX as f32).contains(&num) {
                if num < 0. {
                    exponential -= 1;
                    num *= 10.;
                } else {
                    exponential += 1;
                    num /= 10.;
                }
            }
        }
        if neg {
            num *= -1.
        }

        F32(exponential, num as i32)
    }
}
// TODO:
// impl Add for F32 {
//     type Output = F32;
//     fn add(self, rhs: Self) -> Self::Output {
//
//     }
// }
