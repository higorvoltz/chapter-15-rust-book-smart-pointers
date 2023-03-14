use crate::List::{Cons, Nil};

enum List {
  Cons(i32, Box<List>),
  Nil,
}


fn main() {
  // Using Box<T> to Point to Data on the Heap
  let b = Box::new(5);
  println!("b value is: {}", b);

  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}