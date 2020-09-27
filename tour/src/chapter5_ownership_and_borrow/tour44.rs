struct Foo {
    x: i32,
}

/*
Rustでは、スコープの終わりを
リソースのデストラクトの開放の場所として使用します。
このデストラクトと開放のことをドロップ(drop)と呼びます。

メモリの詳細
* Rust にはガベージコレクションがありません。
* C++ では Resource Aquisition Is Initialization(RAII)
 「リソース取得は初期化である」よも呼ばれています。
*/
fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 13 };

    println!("{}", foo_a.x);

    println!("{}", foo_b.x);
    // foo_b は ここで drop
    // foo_a は ここで drop
}
