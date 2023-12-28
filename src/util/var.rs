// 定数は、定数式しかセットできない。関数の結果や、実行時に評価される値は設定できない
const MAX_POINTS: u32 = 2147483647; // 32ビットの最大値

pub fn main() {
    let x = 123;
    let y = 40;
    let z = x + y;
    println!("{}", z);
    // 以下、cannot assign twice to immutable variableエラーが発生する。
    // 変数は、代入した値を変えられないため
    // z = x -y;

    // 変数名の前にmutキーワードを付けることで、可変にできる。
    let mut zz = x + y;
    println!("{}", zz);
    // より小さいスコープを持つブロックになる。
    {
      // ブロック内で、zzはマスキングされる。
      // シャドーイングを行うことで、異なる変数名を考えなくて良い
      let zz = 7;
      println!("{}", zz);
    }
    // ブロックを抜けると、zzは元の値に戻る
    println!("{}", zz);
    zz = x - y;
    println!("{}", zz);

    println!("{}", MAX_POINTS)
}
