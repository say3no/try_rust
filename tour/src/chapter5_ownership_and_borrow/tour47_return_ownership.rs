struct Foo {
    x: i32,
}

// 下記の -> Foo っていうのは、
// Fooを返すというよりFooの所有権譲渡というニュアンスのほうが正しいのかもしれない。
fn do_something() -> Foo {
    Foo { x: 42 }
    // 所有権は外に移動
}

fn main() {
    let foo = do_something();
    // foo は 所有者になれる。
    // 関数のスコープ終端なので、 foo はドロップされる

    let hoge = Foo { x: 444 };
    // hoge = do_something() // mut じゃないので、fugaを別のモノで上書きできない

    let mut fuga = Foo { x: 3333 };
    println!("{}", fuga.x);
    fuga = do_something(); // mutなので、fooの中身を別のもので上書きできる
    println!("{}", fuga.x);
}
