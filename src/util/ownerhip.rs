use std::f64::consts::PI;
use std::fmt;


pub fn main() {
  copy_literal();
  copy_str();
  copy_int();
  use_reference();
}

fn copy_literal() {
  // ある値の所有権は、1つしかない。
  // msg, msg2は値を所有しているわけではなく、"hello"というリテラル値の参照が渡されている。
  let msg = "hello";
  let msg2 = msg;
  println!("{} {}", msg, msg2)
}

fn copy_str() {
  // 変数に値を代入すると、その値の所有権が設定される。
  let msg = String::from("hello");
  let msg2 = msg;
  // borrow of moved value: 'msg'
  // println!("{}", msg); msgには値の所有権がないため、msgの値を利用できない
  println!("{}", msg2);
}

fn copy_int() {
  // スカラー型の場合、値を代入してもそのコピーが渡されるだけ
  let num = 1000;
  // numの値をnum2に代入しても、num2にはコピーが渡されるだけなので、numの所有権は失われない。
  let num2 = num;
  println!("{} {}", num, num2)
}

struct Circle {
  radius: f64, // 半径
  area: f64, //面積
}

impl Circle {
  // &mut selfによって、可変の参照を渡せる。set_areaは、circleの値を借用している。そのため、set_areaを用いても所有権は移動しない
  fn set_area(&mut self) {
    self.area = PI * self.radius.powi(2)
  }
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle of radius {}, area {}", self.radius, self.area)
  }
}

fn use_reference() {
  let mut circle = Circle{ radius: 5.0, area: 0.0};
  circle.set_area();
  println!("{}", circle.to_string())
}