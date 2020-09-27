struct Foo {
    x: i32,
}

/*
参照は、 & 演算子によってリソースへのアクセスを借用できるようにしてくれます。

参照も、他のリソースと同様にドロップされます。

*/
fn main() {
    let foo = Foo { x: 42 };
    let f = &foo;
    println!("{}", f.x);
    println!("{}", foo.x);

    let mut foo = Foo { x: 33342 };
    println!("{}", f.x); // 42
    println!("{}", foo.x);
    foo = Foo { x: 1111 };
    println!("{}", f.x); // 42
    println!("{}", foo.x);
    // 借りたあとでもとの所有者が色々と変わってしまっても、借りた人は当時の値を保持し続ける

    // f はここでドロップ
    // foo はここでドロップ
}
