fn main() -> () {
    // Tour 14
    let x = 42;
    if x < 42 {
        println!(" 42 より小さい ");
    } else if x == 42 {
        println!(" 42 に等しい ");
    } else {
        println!(" 42 より大きい ");
    }

    // Tour 15
    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }
    println!("{}", x);

    // Tour 16
    let mut x = 0;
    while x != 42 {
        x += 1;
    }
    println!("{}", x);

    // Tour 17
    // like Bash
    for x in 0..5 {
        println!("{}", x);
    }

    for x in 0..=5 {
        println!("{}", x);
    }

    // Tour 18
    let x = 42;
    match x {
        0 => {
            println!("found zero");
        }
        1 | 2 => {
            println!("found 1 or 2");
        }
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100", matched_num);
        }
        _ => {
            println!("found something else!");
        }
    }

    // Tour 19
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "13 を発見";
        }
    };
    println!("loop の戻り値: {}", v);

    // Tour 20
    println!("関数より： {}", example());
}

// Tour 20
fn example() -> i32 {
    let x = 42;
    // Rust の三項式
    let v = if x < 42 { -1 } else { 1 };
    println!("if より: {}", v);

    let food = "ハンバーガー";
    let result = match food {
        "ホットドッグ" => "ホットドッグですよ",
        // 単一の式で値を返す場合、中括弧は省略可能
        _ => "ホットドッグではありません",
    };
    println!("食品の識別：　{}", result);

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("ブロックより: {}", v);

    // Rust で関数の最期から値を返す観葉的な方法
    v + 4
}
