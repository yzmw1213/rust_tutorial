pub fn main() {
    let x = 123;
    let y = 40;
    let z = x + y;
    println!("{}", z);
    // 以下、cannot assign twice to immutable variableエラーが発生する。
    // 変数は、代入した値を変えられないため
    // z = x -y;

    let mut zz = x + y;
    zz = x - y;
    println!("{}", zz);
}
