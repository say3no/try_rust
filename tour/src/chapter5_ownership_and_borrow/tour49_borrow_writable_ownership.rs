struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
}

/*
&mut 演算子を使えば、リソースへの変更可能なアクセスを借用することもできます。
リソースの所有者は、可変な借用の間は、移動や変更ができません。
(参照もだめっぽい)

メモリの詳細:
* データ競合を防止するため、Rustでは同時に二つの変数から値を変更することはできません。
*/
fn main() {
    let mut foo = Foo { x: 42 };
    println!("{}", foo.x); // 42
    let f = &mut foo;

    // 参照するだけでもだめ、
    // 右記もエラー println!("{}", foo.x); // 42

    // 失敗: do_something(foo)はここでエラー。
    // foo は可変に借用されており移動できないため。
    // 失敗: foo.x = 13; はここでエラー
    // foo は可変に借用されている間は変更できないため

    f.x = 13;
    // f はここから先では使用されないため、ここでドロップ
    println!("{}", foo.x); // 23
    foo.x = 7;
    println!("{}", foo.x); // 7
    do_something(foo);
}
