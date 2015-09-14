use std::mem::{transmute_copy, size_of};

/// May only be safely implemented for enums with:
///
///   1. No variant parameters whatsoever.
///   2. No index assignment. For example: `enum T { A = 1, B = 2 }`.
pub unsafe trait AllVariantsTakeNoParameters : Eq + Copy {}

pub struct EnumIter<T: AllVariantsTakeNoParameters> {
  cur : T,
  end : T,
  done: bool,
}

impl<T: AllVariantsTakeNoParameters> EnumIter<T> {
  #[inline]
  pub fn new(start: T, end: T) -> EnumIter<T> {
    assert!(size_of::<T>() == 1
         || size_of::<T>() == 2
         || size_of::<T>() == 4
         || size_of::<T>() == 8);

    EnumIter {
      cur:  start,
      end:  end,
      done: false,
    }
  }
}

macro_rules! next {
  ($ty : ty, $this: expr) => ({
    let ret = $this.cur;
    let cur: $ty = transmute_copy(&ret);
    let next = cur + 1;
    let next = transmute_copy(&next);
    $this.cur = next;
    ret
  })
}

impl<T: AllVariantsTakeNoParameters> Iterator for EnumIter<T> {
  type Item = T;

  #[inline]
  fn next(&mut self) -> Option<T> {
    if self.done {
      None
    } else {
      let ret = unsafe {
        match size_of::<T>() {
          1 => next!(u8 , self),
          2 => next!(u16, self),
          4 => next!(u32, self),
          8 => next!(u64, self),
          _ => unreachable!()
        }
      };
      self.done = ret == self.end;
      Some(ret)
    }
  }
}

/// `enum_iter(Enum::FirstElement, Enum::LastElement)` returns an iterator
/// through the different variants of the enum. `T` must satisfy
/// `AllVariantsTakeNoParameters`, which pretty much just means you built a
/// simple rust enum with no variant parameters and no manual assignment of
/// integer mappings.
pub fn enum_iter<T: AllVariantsTakeNoParameters>(start: T, end: T) -> EnumIter<T> {
  EnumIter::new(start, end)
}

#[cfg(test)]
mod test {
  use super::{enum_iter, AllVariantsTakeNoParameters};

  #[derive(Clone, Copy, PartialEq, Eq, Debug)]
  enum Test {
    One,
    Two,
    Three,
  }

  unsafe impl AllVariantsTakeNoParameters for Test {}

  #[test]
  fn it_works() {
    let mut vals = Vec::new();

    let _ = Test::Two; // ignore dead code warning.

    for x in enum_iter(Test::One, Test::Three) {
      vals.push(x);
    }

    assert_eq!(format!("{:?}", vals), "[One, Two, Three]");
  }
}
