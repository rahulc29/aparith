use std::cmp::Ordering;
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

pub fn add_unsigned(lhs: &[u32], rhs: &[u32]) -> Vec<u32> {
    let (smaller, larger) = if lhs.len() > rhs.len() {
        (rhs, lhs)
    } else {
        (lhs, rhs)
    };
    let mut to_return = bounded_op_unsigned(&larger, &smaller, u32::overflowing_add);
    if let Some(i) = to_return.iter().rposition(|&x| x != 0) {
        let the_length = i + 1;
        to_return.truncate(the_length);
    }
    to_return
}

pub fn compare_unsigned(a: &[u32], b: &[u32]) -> Ordering {
    if a.len() > b.len() {
        return Ordering::Greater;
    }
    if a.len() < b.len() {
        return Ordering::Less;
    }
    let len = a.len();
    for i in (0..len).rev() {
        let a_elem = a[i];
        let b_elem = b[i];
        if a_elem < b_elem {
            return Ordering::Less;
        }
        if a_elem > b_elem {
            return Ordering::Greater;
        }
    }
    return Ordering::Equal;
}

pub fn subtract_unsigned(a: &[u32], b: &[u32]) -> Option<Vec<u32>> {
    return match compare_unsigned(a, b) {
        Ordering::Less => {
            // a - b
            // if a < b result will not be unsigned
            // in that case we return `None`
            None
        }
        Ordering::Equal => {
            Some(Vec::new()) // 0
        }
        Ordering::Greater => {
            Some(bounded_op_unsigned(&a, &b, u32::overflowing_sub))
        }
    };
}

fn bounded_op_unsigned<F>(larger: &[u32], smaller: &[u32], op: F) -> Vec<u32>
    where F: Fn(u32, u32) -> (u32, bool) {
    assert!(larger.len() >= smaller.len());
    let n = larger.len();
    let m = smaller.len();
    let mut to_return = vec![0u32; n];
    let mut overflow_counter = 0u32;
    for i in 0..m {
        let (result, pri_overflow) = op(larger[i], smaller[i]);
        let (result, sec_overflow) = op(result, overflow_counter);
        overflow_counter = match (pri_overflow, sec_overflow) {
            (true, true) => 2,
            (true, false) | (false, true) => 1,
            (false, false) => 0
        };
        to_return[i] = result;
    }
    for i in m..n {
        let (result, overflow) = op(larger[i], overflow_counter);
        overflow_counter = if overflow { 1 } else { 0 };
        to_return[i] = result;
    }
    if overflow_counter != 0 {
        to_return.push(overflow_counter);
    }
    return to_return;
}