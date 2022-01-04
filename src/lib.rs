mod big_int;

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::big_int;
    use crate::big_int::{add_unsigned, BigInt, compare_unsigned};
    use crate::big_int::Sign::Positive;

    #[test]
    fn digit_append_test() {
        let a = vec![!0u32; 5];
        let b = vec![1u32];
        let sum = add_unsigned(&a, &b);
        assert_eq!(*sum.last().unwrap(), 1);
    }

    #[test]
    fn same_length_addition() {
        let a: Vec<u32> = vec![2, 3, 4];
        let b: Vec<u32> = vec![5, 6, 6];
        let sum = add_unsigned(&a, &b);
        assert_eq!(sum, vec![7, 9, 10]);
    }

    #[test]
    fn diff_length_addition() {
        let a: Vec<u32> = vec![3, 0, 3];
        let b: Vec<u32> = vec![3, 0, 3, 5];
        assert_eq!(add_unsigned(&a, &b), vec![6, 0, 6, 5]);
    }

    #[test]
    fn compare_equals() {
        let a: Vec<u32> = vec![0];
        let b: Vec<u32> = vec![0];
        assert_eq!(compare_unsigned(&a, &b), Ordering::Equal);
        let a: Vec<u32> = vec![5; 5];
        let b: Vec<u32> = vec![5; 5];
        assert_eq!(compare_unsigned(&a, &b), Ordering::Equal);
    }

    #[test]
    fn compare_same_length_unequals() {
        let a: Vec<u32> = vec![5; 5];
        let b: Vec<u32> = vec![4; 5];
        assert_eq!(compare_unsigned(&a, &b), Ordering::Greater);
    }
}
