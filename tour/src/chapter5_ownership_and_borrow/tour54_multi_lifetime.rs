struct Foo {
    x: i32,
}

// foo_b と戻り値はライフタイムを共有
// foo_a のライフタイムは別

fn do_something<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
    println!("foo_a.x: {} (in do_something)", foo_a.x);
    println!("foo_b.x: {} (in do_something)", foo_b.x);
    return &foo_b.x;
}

/*
複数のライフタイム

ライフタイム指定子は、関数の引数や戻り地のライフタイムをコンパイラが解決できない場合に、明示的に指定することができます。

*/
fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 12 };
    println!("foo_a.x: {}", foo_a.x);
    println!("foo_b.x: {}", foo_b.x);

    let x = do_something(&foo_a, &foo_b);
    // ここから先は foo_b のライフタイムしか存在しないため、
    // foo_a はここでドロップ
    println!("x: {}", x);

    println!("foo_a.x: {}", foo_a.x);
    println!("foo_b.x: {}", foo_b.x);
    // x はここでドロップ
    // foo_b はここでドロップ
}
