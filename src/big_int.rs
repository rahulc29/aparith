use std::ops::*;

#[repr(u8)]
pub enum Sign {
    Positive,
    Negative,
    Zero,
}

#[derive(Copy)]
pub struct BigInt {
    magnitude: Vec<u32>,
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
        if Sign::Zero = self.sign {
            rhs
        }
        if Sign::Zero = rhs.sign {
            self.clone()
        }

        todo!()
    }
}

fn add_unsigned(lhs: &BigInt, rhs: &BigInt) -> Vec<u32> {
    let (smaller, larger) = if lhs.magnitude.len() > rhs.magnitude.len() {
        (&rhs.magnitude, &lhs.magnitude)
    } else {
        (&lhs.magnitude, &rhs.magnitude)
    };
    let n = smaller.data.len();
    let m = larger.data.len();
    let mut carry = 0u32;
    let mut to_return = vec![0u32; m];
    for i in 0..n {
        let (result, overflow) = smaller[i].overflowing_add(larger[i]);
        to_return[i] = result + carry;
        if overflow {
            carry += 1;
        }
    }
    for i in n..m {
        let (result, overflow) = carry.overflowing_add(larger[i]);
        to_return[i] = result + carry;
        if overflow {
            carry += 1;
        }
    }
    todo!()
}