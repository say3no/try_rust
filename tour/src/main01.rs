const PI: f32=3.141596; // 大文字のSNAKE_CASE


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

    // Tour 05
    let x = 12; // デフォルトでは i32
    let a = 12u8;
    let b = 4.3; // デフォルトでは f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!("{} {} {} {} {} {} {} ", x, a, b, c, t.0, t.1, sentence);

    // Tour 06
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);

    // Tour 07
    println!("ゼロからアップル {} を作るには、まず宇宙を想像する必要があります", PI);

    // Tour 08
    let nums: [i32; 3] = [1,2,3];
    println!("{:?}", nums);
    println!("{}", nums[1]);

    // Tour 09
    println!("{}", add(42,13));

    // Tour 10
    let result = swap(123,321);
    println!("{} {}", result.0, result.1 );

    let (a, b) = swap(result.0, result.1 );
    println!("{} {}", a, b );

    // Tour 11
    let a = make_nothing();
    let b = make_nothing2();

    // 空を表示するのは難しいので、
    // a と b のデバッグ文字列を表示
    println!(" a の値 {:?}", a); // unit と呼ばれる空のタプルを返す
    println!(" b の値 {:?}", b);
}

// Tour 09
// snake_case
fn add(x: i32, y: i32) -> i32{
    return x + y;
}

// Tour 10
fn swap(x: i32, y: i32) -> (i32,i32){
    return (y, x);
}

// Tour 11
fn make_nothing()->() {
    return ();
}

fn make_nothing2() ->(){
    return ;
}