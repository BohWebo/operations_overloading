use std::ops::{Add, Sub, Div, Mul, Rem};

pub trait Num: Copy
+ Default
+ Add<Output=Self>
+ Sub<Output=Self>
+ Div<Output=Self>
+ Mul<Output=Self>
+ Rem<Output=Self>
{
    fn safe_add(self, _rhs: Self) -> Option<Self> {
        None
    }

    fn safe_sub(self, _rhs: Self) -> Option<Self> { None }

    fn safe_mul(self, _rhs: Self) -> Option<Self> {
        None
    }

    fn safe_div(self, _rhs: Self) -> Option<Self> {
        None
    }

    fn safe_rem(self, _rhs: Self) -> Option<Self> {
        None
    }
}

impl Num for u8 {
    fn safe_add(self, rhs: Self) -> Option<Self> {
        u8::checked_add(self, rhs)
    }

    fn safe_sub(self, rhs: Self) -> Option<Self> { u8::checked_sub(self, rhs) }

    fn safe_mul(self, rhs: Self) -> Option<Self> {
        u8::checked_mul(self, rhs)
    }

    fn safe_div(self, rhs: u8) -> Option<u8> {
        u8::checked_div(self, rhs)
    }

    fn safe_rem(self, rhs: u8) -> Option<u8> {
        u8::checked_rem(self, rhs)
    }
}

impl Num for i8 {
    fn safe_add(self, rhs: Self) -> Option<Self> {
        i8::checked_add(self, rhs)
    }

    fn safe_sub(self, rhs: Self) -> Option<Self> {
        i8::checked_sub(self, rhs)
    }

    fn safe_mul(self, rhs: Self) -> Option<Self> {
        i8::checked_mul(self, rhs)
    }

    fn safe_div(self, rhs: i8) -> Option<i8> {
        i8::checked_div(self, rhs)
    }

    fn safe_rem(self, rhs: i8) -> Option<i8> {
        i8::checked_rem(self, rhs)
    }
}

impl Num for u16 {
    fn safe_add(self, rhs: Self) -> Option<Self> {
        u16::checked_add(self, rhs)
    }

    fn safe_sub(self, rhs: Self) -> Option<Self> {
        u16::checked_sub(self, rhs)
    }

    fn safe_mul(self, rhs: Self) -> Option<Self> {
        u16::checked_mul(self, rhs)
    }

    fn safe_div(self, rhs: u16) -> Option<u16> {
        u16::checked_div(self, rhs)
    }

    fn safe_rem(self, rhs: u16) -> Option<u16> {
        u16::checked_rem(self, rhs)
    }
}

impl Num for i16 {
    fn safe_add(self, rhs: Self) -> Option<Self> {
        i16::checked_add(self, rhs)
    }

    fn safe_sub(self, rhs: Self) -> Option<Self> {
        i16::checked_sub(self, rhs)
    }

    fn safe_mul(self, rhs: Self) -> Option<Self> {
        i16::checked_mul(self, rhs)
    }

    fn safe_div(self, rhs: i16) -> Option<i16> {
        i16::checked_div(self, rhs)
    }

    fn safe_rem(self, rhs: i16) -> Option<i16> {
        i16::checked_rem(self, rhs)
    }
}

impl Num for u32 {
    fn safe_add(self, rhs: Self) -> Option<Self> {
        u32::checked_add(self, rhs)
    }

    fn safe_sub(self, rhs: Self) -> Option<Self> {
        u32::checked_sub(self, rhs)
    }

    fn safe_mul(self, rhs: Self) -> Option<Self> {
        u32::checked_mul(self, rhs)
    }

    fn safe_div(self, rhs: u32) -> Option<u32> {
        u32::checked_div(self, rhs)
    }

    fn safe_rem(self, rhs: u32) -> Option<u32> {
        u32::checked_rem(self, rhs)
    }
}

impl Num for i32 {
    fn safe_add(self, rhs: Self) -> Option<Self> {
        i32::checked_add(self, rhs)
    }

    fn safe_sub(self, rhs: Self) -> Option<Self> {
        i32::checked_sub(self, rhs)
    }

    fn safe_mul(self, rhs: Self) -> Option<Self> {
        i32::checked_mul(self, rhs)
    }

    fn safe_div(self, rhs: i32) -> Option<i32> {
        i32::checked_div(self, rhs)
    }

    fn safe_rem(self, rhs: i32) -> Option<i32> {
        i32::checked_rem(self, rhs)
    }
}

impl Num for u64 {
    fn safe_add(self, rhs: Self) -> Option<Self> {
        u64::checked_add(self, rhs)
    }

    fn safe_sub(self, rhs: Self) -> Option<Self> {
        u64::checked_sub(self, rhs)
    }

    fn safe_mul(self, rhs: Self) -> Option<Self> {
        u64::checked_mul(self, rhs)
    }

    fn safe_div(self, rhs: u64) -> Option<u64> {
        u64::checked_div(self, rhs)
    }

    fn safe_rem(self, rhs: u64) -> Option<u64> {
        u64::checked_rem(self, rhs)
    }
}

impl Num for i64 {
    fn safe_add(self, rhs: Self) -> Option<Self> { i64::checked_add(self, rhs) }

    fn safe_sub(self, rhs: Self) -> Option<Self> { i64::checked_sub(self, rhs) }

    fn safe_mul(self, rhs: Self) -> Option<Self> {
        i64::checked_mul(self, rhs)
    }

    fn safe_div(self, rhs: i64) -> Option<i64> {
        i64::checked_div(self, rhs)
    }

    fn safe_rem(self, rhs: i64) -> Option<i64> {
        i64::checked_rem(self, rhs)
    }
}

impl Num for f32 {
    fn safe_div(self, rhs: f32) -> Option<f32> {
        if rhs == 0_f32 || self == 0_f32 {
            None
        } else {
            Some(self / rhs)
        }
    }

    fn safe_rem(self, rhs: f32) -> Option<f32> {
        if rhs == 0_f32 || self == 0_f32 {
            None
        } else {
            Some(self % rhs)
        }
    }
}

impl Num for f64 {
    fn safe_div(self, rhs: f64) -> Option<f64> {
        if rhs == 0_f64 || self == 0_f64 {
            None
        } else {
            Some(self / rhs)
        }
    }

    fn safe_rem(self, rhs: f64) -> Option<f64> {
        if rhs == 0_f64 || self == 0_f64 {
            None
        } else {
            Some(self % rhs)
        }
    }
}
