use std::any::type_name;

pub fn main() {
  numeric_literal()
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

