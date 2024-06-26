#![no_std]

pub trait COto<const C: usize> {
    fn coto(&self) -> [u8; C];
}

pub trait COtoUTF8<const O: usize> {
    fn coto_utf8(&self) -> [u8; O];
}

impl COto<1> for u8 {
    fn coto(&self) -> [u8; 1] {
        self.to_le_bytes()
    }
}

impl COto<4> for f32 {
    fn coto(&self) -> [u8; 4] {
        self.to_le_bytes()
    }
}

impl COtoUTF8<3> for u8 {
    fn coto_utf8(&self) -> [u8; 3] {
        let mut digit: [u8; 3] = [0; 3];
        let mut n = *self;
        let mut i = 2;

        while n > 0 {
            digit[i] = (n % 10) as u8;
            n /= 10;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        for i in 0..digit.len() {
            digit[i] += 48
        }
        digit
    }
}
//Future plan
// struct F32((i8, i8, u16));

impl COtoUTF8<9> for f32 {
    fn coto_utf8(&self) -> [u8; 9] {
        if *self == f32::INFINITY {
            *b"+INFINITY"
        } else if *self == f32::NEG_INFINITY {
            *b"-INFINITY"
            // [43, 73, 78, 70, 73, 78, 73, 84, 89]
        } else {
            let mut num = *self;
            let mut exponential: i8 = 0;
            while num < 0. || num > 10. {
                if num < 0. {
                    exponential -= 1;
                    num *= 10.;
                } else {
                    exponential += 1;
                    num /= 10.;
                }
            }
            // let coto
            let cotoexpo = exponential.coto_utf8();
            match exponential {
                // i8::MIN..=-100 | 100..=i8::MAX => {}
                -99..=-10 | 10..=99 => {
                    let cotonum = ((num * 1000.) as i16).coto_utf8();
                    [
                        cotonum[0],
                        cotonum[2],
                        0x2e,
                        cotonum[3],
                        cotonum[4],
                        cotonum[5],
                        0x45,
                        cotoexpo[2],
                        cotoexpo[3],
                    ]
                }
                -9..=-1 | 1..=9 => {
                    let cotonum = ((num * 10000.) as i32).coto_utf8();
                    [
                        cotonum[0],
                        cotonum[6],
                        0x2e,
                        cotonum[7],
                        cotonum[8],
                        cotonum[9],
                        cotonum[10],
                        0x45,
                        cotoexpo[3],
                    ]
                }
                _ => {
                    let cotonum = ((num * 100000.) as i32).coto_utf8();
                    [
                        cotonum[0],
                        cotonum[4],
                        cotonum[5],
                        0x2e,
                        cotonum[6],
                        cotonum[7],
                        cotonum[8],
                        cotonum[9],
                        cotonum[10],
                    ]
                }
            }
        }
    }
}

impl COtoUTF8<4> for i8 {
    fn coto_utf8(&self) -> [u8; 4] {
        let mut digit: [u8; 4] = [0; 4];
        let mut n = self.abs();
        let mut i = 3;

        while n > 0 {
            digit[i] = (n % 10) as u8;
            n /= 10;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        for i in 0..digit.len() {
            digit[i] += 48
        }
        if self.is_positive() {
            digit[0] = 0x2b
        } else {
            digit[0] = 0x2d
        }
        digit
    }
}

impl COtoUTF8<6> for i16 {
    fn coto_utf8(&self) -> [u8; 6] {
        let mut digit: [u8; 6] = [0; 6];
        let mut n = self.abs();
        let mut i = 5;

        while n > 0 {
            digit[i] = (n % 10) as u8;
            n /= 10;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        for i in 0..digit.len() {
            digit[i] += 48
        }
        if self.is_positive() {
            digit[0] = 0x2b
        } else {
            digit[0] = 0x2d
        }
        digit
    }
}

impl COtoUTF8<11> for i32 {
    fn coto_utf8(&self) -> [u8; 11] {
        let mut digit: [u8; 11] = [0; 11];
        let mut n = self.abs();
        let mut i = 10;

        while n > 0 {
            digit[i] = (n % 10) as u8;
            n /= 10;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        for i in 0..digit.len() {
            digit[i] += 48
        }
        if self.is_positive() {
            digit[0] = 0x2b
        } else {
            digit[0] = 0x2d
        }
        digit
    }
}

impl COtoUTF8<10> for u32 {
    fn coto_utf8(&self) -> [u8; 10] {
        let mut digit: [u8; 10] = [0; 10];
        let mut n = *self;
        let mut i = 9;

        while n > 0 {
            digit[i] = (n % 10) as u8;
            n /= 10;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        for i in 0..digit.len() {
            digit[i] += 48
        }
        digit
    }
}

impl COtoUTF8<5> for u16 {
    fn coto_utf8(&self) -> [u8; 5] {
        let mut digit: [u8; 5] = [0; 5];
        let mut n = *self;
        let mut i = 4;

        while n > 0 {
            digit[i] = (n % 10) as u8;
            n /= 10;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        for i in 0..digit.len() {
            digit[i] += 48
        }
        digit
    }
}

// Future plan
/*
trait LastThreeDigit {
    fn lastthreedigit<const C: usize, const O: usize>(cot0_utf8: &dyn COtoUTF8<C>) -> [u8; 3];
}

trait LastFiveDigit {
    fn lastthreedigit<const C: usize, const O: usize>(cot0_utf8: &dyn COtoUTF8<C>) -> [u8; 3];
}
*/
