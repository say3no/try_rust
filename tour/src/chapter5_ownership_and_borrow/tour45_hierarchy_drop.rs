struct Bar {
    x: i32,
}

struct Foo {
    bar: Bar,
}

/* 構造体がドロップされると、まず構造体自体がドロップされ、
次にその子要素が個別に削除されます。
* メモリを自動的に開放することで、メモリリークを軽減できます
* メモリリソースのドロップは一度しかできません。
*/
fn main() {
    let foo = Foo { bar: Bar { x: 42 } };
    println!("{}", foo.bar.x);
    // foo が最初にドロップ
    // 次に foo.bar が
}
