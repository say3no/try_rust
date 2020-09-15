fn main(){
    // Tour 03
    // x の型を推論
    let x = 13;
    println!("{}", x);

    // x の型を指定
    let x: f64 = 3.14159;
    println!("{}", x);

    // 宣言のあとで初期化
    let x;
    x =0;
    println!("{}", x);

    // Tour 04
    let mut x = 42;
    println!("{}",x);
    x = 13;
    println!("{}",x);

}