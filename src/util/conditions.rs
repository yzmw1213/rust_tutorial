pub fn main() {
  use_if_else();
  use_loop();
  use_for();
}

fn use_if_else() {
  let n = 5;
  if n < 0 {
    println!("{} is negative", n )
  } else if n > 0 {
    println!("{} is positive", n )
  } else {
    println!("{} is zero", n )
  }

  let big_n =
    if n % 2 > 0 {
      println!("{} is odd number, increase 1000", n );
      n + 1000
    } else {
      println!("{} is even number, increase ten-fold", n );
      n * 1000
    };
  println!("{} -> {}", n, big_n);
}

fn use_loop() {
  use_loop_example1();
}

fn use_loop_example1() {
  let mut count = 0;
  loop {
    count+= 1;
    if count > 10 {
      break
    }
  }
  println!("Loop finished!  {}", count);
}

fn use_for() {
  for n in 1..10 {
    let result: &str;
    if n % 2 > 0 {
      result = "奇数";
    } else {
      result = "偶数";
    }
    println!("{} は {}", n, result)
  }
}