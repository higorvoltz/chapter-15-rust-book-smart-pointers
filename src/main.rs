use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
  Cons(i32, Box<List>),
  Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn main() {
  // Using Box<T> to Point to Data on the Heap
  let b = Box::new(5);
  println!("b value is: {}", b);

  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

  //Treating Smart Pointers Like Regular References with the Deref Trait
  let x = 5;
  let y = &x;

  assert_eq!(5, x);
  assert_eq!(5, *y); //
  // assert_eq!(5, y); //error

  //Using Box<T> Like a Reference
  let a = 1;
  let bb = Box::new(a);

  assert_eq!(1, a);
  // assert_eq!(1, *a); // error[E0614]: type `{integer}` cannot be dereferenced

  // Defining Our Own Smart Pointer
  let z = 5;
  let w = MyBox::new(z);

  assert_eq!(5, *w);

  assert_eq!(5, *(w.deref()));

  // Implicit Deref Coercions with Functions and Methods
  fn hello(name: &str) {
    println!("Hello, {name}!");
  }

  let rust = MyBox::new(String::from("Rust"));
  hello(&rust);

  hello(&(*rust)[..]);

}