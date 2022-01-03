use std::ops::*;

#[repr(u8)]
pub enum Sign {
    Positive,
    Negative,
    Zero,
}

pub struct BigInt {
    pub magnitude: Vec<u32>,
    sign: Sign,
}

impl BigInt {
    pub fn new() -> Self {
        Self {
            sign: Sign::Zero,
            magnitude: Vec::new(),
        }
    }
}

impl From<i32> for BigInt {
    fn from(number: i32) -> Self {
        let sign = match number {
            0 => Sign::Zero,
            number => {
                if number < 0 {
                    Sign::Negative
                } else {
                    Sign::Positive
                }
            }
        };
        let number = number.abs() as u32;
        Self {
            sign,
            magnitude: vec![number],
        }
    }
}

impl Add<BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: BigInt) -> Self::Output {
        todo!()
    }
}

pub fn add_unsigned(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
    let (smaller, larger) = if lhs.len() > rhs.len() {
        (rhs, lhs)
    } else {
        (lhs, rhs)
    };
    let n = smaller.len();
    let m = larger.len();
    let mut carry = 0u32;
    let mut to_return = vec![0u32; m];
    for i in 0..n {
        let (result, pri_overflow) = smaller[i].overflowing_add(larger[i]);
        let (result, sec_overflow) = result.overflowing_add(carry);
        to_return[i] = result;
        carry = match (pri_overflow, sec_overflow) {
            (true, true) => 2,
            (true, false) | (false, true) => 1,
            (false, false) => 0
        };
    }
    for i in n..m {
        let (result, overflow) = carry.overflowing_add(larger[i]);
        to_return[i] = result;
        if overflow {
            carry = 1;
        } else {
            carry = 0;
        }
    }
    if carry != 0 {
        to_return.push(carry);
    }
    to_return
}