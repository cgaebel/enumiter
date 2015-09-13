EnumIter
========

A simple iterator through the different variants in an enum.

Example
=======

```rust
  use enumiter::{enum_iter, AllVariantsTakeNoParameters};

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
```