extern crate num_traits;

use num_traits::ops::wrapping::WrappingAdd;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

pub struct WrappedNum<T: WrappingAdd + Ord + Copy + PartialEq> {
    num: T,
    counter: i64,
}

impl<T: WrappingAdd + Ord + Copy + PartialEq> WrappedNum<T> {
    pub fn new(init: T) -> WrappedNum<T> {
        WrappedNum {
            num: init,
            counter: 0i64,
        }
    }

    pub fn get(&self) -> T {
        self.num
    }

    pub fn wrapping_add(&mut self, inc: T) {
        let sum = self.num.wrapping_add(&inc);
        if sum < self.num {
            self.counter += 1;
        }
        self.num = sum;
    }
}

impl<T: WrappingAdd + Ord + Copy + PartialEq> PartialEq for WrappedNum<T> {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.counter == other.counter
    }
}

impl<T: WrappingAdd + Ord + Copy + PartialEq> PartialOrd for WrappedNum<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.counter.cmp(&other.counter) {
            Ordering::Equal => Some(self.num.cmp(&other.num)),
            other => Some(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_wrapped_num() {
        let _ = WrappedNum::new(0u8);
    }

    #[test]
    fn returns_number_value() {
        let wn = WrappedNum::new(123u8);
        assert_eq!(123u8, wn.get());
    }

    #[test]
    fn can_wrapped_add() {
        let mut wn = WrappedNum::new(200u8);
        wn.wrapping_add(70u8);
        assert_eq!(14u8, wn.get());
    }

    #[test]
    fn non_wrapped_cmp_works_gt() {
        let left = WrappedNum::new(100u8);
        let mut right = WrappedNum::new(90u8);
        right.wrapping_add(20u8);
        assert!(left < right);
    }

    #[test]
    fn non_wrapped_cmp_works_eq() {
        let left = WrappedNum::new(100u8);
        let mut right = WrappedNum::new(90u8);
        right.wrapping_add(10u8);
        assert!(left == right);
    }

    #[test]
    fn wrapped_cmp_works() {
        let left = WrappedNum::new(200u8);
        let mut right = WrappedNum::new(200u8);
        right.wrapping_add(100u8);
        assert!(left.get() > right.get());
        assert!(left < right);
    }

    #[test]
    // FIXTHIS
    fn wrapped_negative_cmp_works() {
        let left = WrappedNum::new(-100i8);
        let mut right = WrappedNum::new(-100i8);

        right.wrapping_add(-128i8);
        right.wrapping_add(-120i8);
        // right.wrapping_add(-8i8);
        assert!(left.get() < right.get());
        assert!(left < right);
    }
}
