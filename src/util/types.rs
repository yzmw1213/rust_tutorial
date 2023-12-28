use std::any::type_name;
use std::fmt;

pub fn main() {
  numeric_literal();
  println!("{}",convert_i64_to_str(1000i64));

  let circle = Circle{ radius: 5};
  println!("{}", circle.to_string())
}

// 数値型リテラル
fn numeric_literal() {
  // サフィックスにより型を指定できる。
  // サフィックスを指定したリテラル。型は使用方法に依存する。
  let x = 1u8;
  let y = 2u32;
  let z = 3f32;
  println!("{}, Type of x: {}", x, get_type(x)); // u8
  println!("{}, Type of y: {}", y, get_type(y)); // u32
  println!("{}, Type of z: {}", z, get_type(z)); // f32

  // サフィックスを指定しないリテラル。型は使用方法に依存する。
  let i = 1;
  let f= 1.0;
  println!("{}, Type of i: {}", i, get_type(i)); // i32
  println!("{}, Type of f: {}", f, get_type(f)); // f64
}

// 与えられた型の名前を文字列で返す
fn get_type<T>(_: T) -> String{
  let a = type_name::<T>();
  return a.to_string();
}

// to_stringを呼び出すよりも、fmt::Displayトレイトを実装する方が好ましい。
fn convert_i64_to_str(val: i64) -> String {
  return val.to_string()
}

struct Circle {
  radius: i64
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle of radius {}", self.radius)
  }
}
