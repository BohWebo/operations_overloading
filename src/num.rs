use std::ops::{Add, Sub, Div, Mul, Rem};

pub trait Num: Copy
+ Add<Output=Self>
+ Sub<Output=Self>
+ Div<Output=Self>
+ Mul<Output=Self>
+ Rem<Output=Self>
{}

impl Num for u8 {}

impl Num for i8 {}

impl Num for u16 {}

impl Num for i16 {}

impl Num for u32 {}

impl Num for i32 {}

impl Num for u64 {}

impl Num for i64 {}
