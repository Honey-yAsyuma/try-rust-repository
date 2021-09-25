#![allow(overflowing_literals)]
#![allow(dead_code)]

use std::mem;

// pub fn libfunc() {
//   let a = "test";
//   println!("{}", a);
// }

// mod modules;

// use crate::modules::my_mod::nest::ABox;
// use actix_web::{get, web, App, HttpServer, Responder};
// use modules::my_mod::nest::BBox as B;
// use modules::psql;

// #[get("/{id}/{name}/index.html")]
// async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//   format!("Hello {}! id:{}", name, id)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//   libfunc();
//   modules::my_mod::modfunc();
//   let ss = modules::my_mod::nest::ABox {
//       contents: "tesssss",
//   };

//   println!("{}", ss.contents);

//   let lll = ABox { contents: "llll" };
//   println!("{}", lll.contents);

//   let zzz = B { bcontents: "zzz" };
//   println!("{}", zzz.bcontents);

//   psql::init();
//   HttpServer::new(|| App::new().service(index))
//       // .bind("0.0.0.0:8888")?
//       .bind("localhost:8888")?
//       .run()
//       .await
// }

fn main() {
  // array_();
  // struct_();
  // enum_();
  // val_();
  // ownership_();
  // ref_();
  // closures_();
  // hof_();
  // trait_();
  // err_option();
  // option_unpack();
  // option_map();
  // err_result();
  // multiple_errs();
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

#[derive(Debug)]
struct Sheep {
  naked: bool,
  name: &'static str,
}

struct Cow {}
struct Dog {}

trait Animal {
  fn new(name: &'static str) -> Self;

  fn name(&self) -> &'static str;
  fn noise(&self) -> &'static str;

  fn talk(&self) {
    println!("{}", self.name());
  }
}

trait LoveAnimal {
  fn noise(&self) -> &'static str;
}

impl Sheep {
  fn is_naked(&self) -> bool {
    self.naked
  }

  fn shear(&mut self) {
    if self.is_naked() {
      println!("{}", self.name());
    } else {
      println!("{}", self.name);
      self.naked = true;
    }
  }
}

impl Animal for Sheep {
  fn new(name: &'static str) -> Sheep {
    Sheep {
      name: name,
      naked: false,
    }
  }

  fn noise(&self) -> &'static str {
    "baaah!"
  }

  fn name(&self) -> &'static str {
    self.name
  }
}

impl LoveAnimal for Cow {
  fn noise(&self) -> &'static str {
    "mooooo!"
  }
}

impl LoveAnimal for Dog {
  fn noise(&self) -> &'static str {
    "bow wow"
  }
}

fn random_animal(ran: f64) -> Box<dyn LoveAnimal> {
  if ran < 0.5 {
    Box::new(Dog {})
  } else {
    Box::new(Cow {})
  }
}

fn trait_() {
  let mut dolly: Sheep = Animal::new("Dolly");
  println!("{:?}", dolly);

  let random_number = 0.623;
  let animal = random_animal(random_number);
  println!("{}", animal.noise());
}

fn give_princess(gift: Option<&str>) {
  let inside = gift.unwrap();
  if inside == "snake" {
    panic!("aaaaaa");
  }
  println!("I love {}s!!!!!", inside);
}

fn give_commoner(gift: Option<&str>) {
  match gift {
    Some("snake") => println!("putting"),
    Some(inner) => println!("safe {}", inner),
    None => println!("No gift"),
  }
}

fn next_birthday(current_age: Option<u8>) -> Option<String> {
  let next_age: u8 = current_age?;
  Some(format!("next year {}", next_age))
}

fn err_option() {
  let food = Some("cabbage");
  let snake = Some("snake");
  let void = None;

  give_commoner(food);
  give_commoner(snake);
  give_commoner(void);

  let bird = Some("robin");
  let nothing = None;

  give_princess(bird);
  give_princess(nothing);

  println!("err end");
}

struct Phone {
  address: Option<Address>,
}

#[derive(Clone, Copy)]
struct Address {
  phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
  area_code: Option<u8>,
  number: u32,
}

impl Phone {
  fn display_number(&self) -> Option<u8> {
    self.address?.phone_number?.area_code
  }
}

fn option_unpack() {
  let p = Phone {
    address: Some(Address {
      phone_number: Some(PhoneNumber {
        area_code: Some(81),
        number: 00009999,
      }),
    }),
  };

  assert_eq!(p.display_number(), Some(81));
}

#[derive(Debug)]
enum Food {
  Apple,
  Carrot,
  Potato,
}

#[derive(Debug)]
// TODO
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
  match food {
    Some(food) => Some(Peeled(food)),
    None => None,
  }
}
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
  match peeled {
    Some(Peeled(food)) => Some(Chopped(food)),
    None => None,
  }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
  chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
  food
    .map(|f| Peeled(f))
    .map(|Peeled(f)| Chopped(f))
    .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
  match food {
    Some(food) => println!("I love{:?}", food),
    None => println!("Oh no"),
  }
}

fn option_map() {
  let apple = Some(Food::Apple);
  let carrot = Some(Food::Carrot);
  let potato = None;

  println!("{:?}", Peeled(Food::Carrot));

  let cooked_apple = cook(chop(peel(apple)));
  let cooked_carrot = cook(chop(peel(carrot)));

  let cooked_potato = process(potato);

  eat(cooked_apple);
  eat(cooked_carrot);
  eat(cooked_potato);
}

type Res<T> = Result<T, ParseIntError>;

fn multiply(first: &str, second: &str) -> Res<i32> {
  // let f = first.parse::<i32>().unwrap();
  // let s = second.parse::<i32>().unwrap();
  // f + s

  // match first.parse::<i32>() {
  //   Ok(first) => match second.parse::<i32>() {
  //     Ok(second) => Ok(first * second),
  //     Err(e) => Err(e),
  //   },
  //   Err(e) => Err(e),
  // }

  // let f = match first.parse::<i32>() {
  //   Ok(first) => first,
  //   Err(e) => return Err(e),
  // };
  // let s = match second.parse::<i32>() {
  //   Ok(second) => second,
  //   Err(e) => return Err(e),
  // };
  // Ok(f * s)

  // first
  //   .parse::<i32>()
  //   .and_then(|f| second.parse::<i32>().map(|s| f * s))

  let f = first.parse::<i32>()?;
  let s = second.parse::<i32>()?;

  Ok(f * s)
}

fn print(result: Res<i32>) {
  match result {
    Ok(n) => println!("n is {}", n),
    Err(e) => println!("Error: {}", e),
  }
}

use std::num::ParseIntError;

fn err_result() {
  let twenty = multiply("10", "12");
  // println!("double is {}", twenty);

  print(twenty);
  let tt = multiply("t", "2");
  // println!("double is {}", tt);

  print(tt);
}

// fn double_first(vec: Vec<&str>) -> i32 {
//   let first = vec.first().unwrap();
//   2 * first.parse::<i32>().unwrap()
// }

// fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
//   vec.first().map(|first| first.parse::<i32>().map(|n| n * 2))
// }

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
  let opt = vec.first().map(|first| first.parse::<i32>().map(|n| n * 2));

  opt.map_or(Ok(None), |r| r.map(Some))
}

fn multiple_errs() {
  let numbers = vec!["42", "93", "18"];
  let empty: Vec<&str> = vec![];
  let strings = vec!["tofu", "93", "18"];

  println!("{:?}", double_first(numbers));
  println!("{:?}", double_first(empty));
  println!("{:?}", double_first(strings));
}
