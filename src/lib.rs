extern crate num_traits;

use num_traits::ops::wrapping::WrappingAdd;
use std::cmp::Ord;

pub struct WrappedNum<T: WrappingAdd + Ord> {
    num: T,
    counter: i64,
}

impl<T: WrappingAdd + Ord> WrappedNum<T> {
    pub fn new(init: T) -> WrappedNum<T> {
        WrappedNum {
            num: init,
            counter: 0i64,
        }
    }

    pub fn get(&self) -> &T {
        &self.num
    }

    pub fn wrapping_add(&mut self, inc: T) {
        let sum = self.num.wrapping_add(&inc);
        if sum < self.num {
            self.counter += 1;
        }
        self.num = sum;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_wrapped_num() {
        let wn = WrappedNum::new(0u8);
    }

    #[test]
    fn returns_number_value() {
        let wn = WrappedNum::new(123u8);
        assert_eq!(&123u8, wn.get());
    }

    #[test]
    fn can_wrapped_add() {
        let mut wn = WrappedNum::new(200u8);
        wn.wrapping_add(70u8);
        assert_eq!(&14u8, wn.get());
    }
}
