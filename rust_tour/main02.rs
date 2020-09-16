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
        x +=1;
        if x ==42 {
            break;
        }
    }
    println!("{}", x );

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
        println!("{}",x);
    }

}