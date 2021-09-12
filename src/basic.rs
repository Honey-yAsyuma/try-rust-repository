#![allow(overflowing_literals)]
#![allow(dead_code)]

use std::mem;

fn main() {
  // array_();
  // struct_();
  // enum_();
  // val_();
  // ownership_();
  // ref_();
  // closures_();
  hof_();
}

// borrows a slice
fn analyze_slice(slice: &[i32]) {
  println!("analize");
  println!("{}", slice[0]);
  println!("{}", slice.len());
}

fn array_() {
  println!("aaa");

  let a = 12;
  let b = a;

  println!("a is {a} b is {b}", a = a, b = b);

  let xs = [0; 10];
  println!("{}", xs[0]);
  println!("{}", xs.len());

  println!("{}", mem::size_of_val(&xs));
  analyze_slice(&xs[1..4]);
}

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

struct TwoPoint {
  p1: Point,
  p2: Point,
}

fn struct_() {
  let point = Point { x: 2.3, y: 3.4 };
  let pair = Pair(3, 4.44);

  let _nil = Nil;

  let a = Point { ..point };
  println!("{:?}", a);

  let Point { x: top, y: left } = point;
  println!("{x}, {y}", x = top, y = left);
  let Pair(integer, decimal) = pair;
  println!("{:?}, {}", integer, decimal);
}

#[allow(dead_code)]
enum WebEvent {
  PageLoad,
  PageUnload,
  KeyPress(char),
  Paste(String),

  Click { x: i64, y: i64 },
}

#[allow(dead_code)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
  Add,
  Subtract,
}

#[allow(dead_code)]
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn inspect(event: WebEvent) {
  use crate::WebEvent::*;

  match event {
    PageLoad => println!("page loaded"),
    PageUnload => println!("page unloaded"),
    KeyPress(c) => println!("press {}", c),
    Paste(s) => println!("paste {}", s),
    Click { x, y } => println!("clicked at x={}, y ={}", x, y),
  }
}
#[allow(dead_code)]
enum Number {
  Zero,
  One,
  Two,
}

fn enum_() {
  use crate::Number::*;
  use crate::WebEvent::*;

  let pressed = KeyPress('x');
  let loaded = One;

  inspect(pressed);

  println!("{}", loaded as i32);

  let x = Operations::Add;
}

fn val_() {
  let mut x = 5;
  const MAX: u128 = 1000000000000000000;
  println!("{}", x);
  x = 12;
  println!("{}", x);
  println!("{}", MAX);

  let y: u128 = 500000000000;
  println!("{}", y);
  let y = y as u8;
  println!("{}", y);

  let five = 5;
  let z = {
    let five = 2;
    five * 2
  };

  fn six() -> i32 {
    6;
    9
  }

  println!("{}", z);
  println!("{}", {
    let v = 100;
    v
  });

  let w = if true { 12 } else { 24 };

  println!("{}", w);

  let a = [10, 20, 30];

  for e in a.iter() {
    println!("{}", e);
  }
}

fn print_string(stri: String) -> String {
  println!("{}", stri);
  stri
}

fn ownership_() {
  let mut s = String::from("hello");
  let ss = s.clone();

  let aa = "aa";

  s.push_str(", world");

  println!("{}", s);
  println!("{}", ss);

  let sss = print_string(s);
  println!("{}", sss);
}

fn ref_() {}

fn closures_() {
  fn func(i: i32) -> i32 {
    i + 1
  }

  let closure_annotated = |i: i32| -> i32 { i + 1 };
  let closure_inferred = |i| i + 1;

  let i = 1;

  println!("{}", func(i));
  println!("{}", closure_annotated(i));
  println!("{}", closure_inferred(i));
}

fn hof_() {
  let is_odd = |n: u32| -> bool { n % 2 == 1 };

  let upper = 1000;

  let sum_of_squared_odd_numbers: u32 = (0..)
    .map(|n| n * n)
    .take_while(|&n_squared| n_squared < upper)
    .filter(|&n_squared| is_odd(n_squared))
    .fold(0, |acc, n_squared| acc + n_squared);
  println!("{}", sum_of_squared_odd_numbers);
}
