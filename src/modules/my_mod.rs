pub fn modfunc() {
  let a = 12;
  println!("{}", a);
}

pub mod nest {
  pub struct ABox<T> {
    pub contents: T,
  }  pub struct BBox<T> {
    pub bcontents: T,
  }
}
