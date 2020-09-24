struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    return &a.x; // 参照の参照
}

fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x はここでドロップされるため、普遍な参照が作成可能

    let y = do_something(&foo);
    println!("{}", y);
    // y はここでドロップ
    // foo はここでドロップ
}
