struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x)
}

/* 所有者が関数の実引数として渡されると、
所有権は関数の仮引数に移動(move)します。

メモリの詳細:
 * 移動している間、所有者の値のスタックメモリは、
 * 関数呼び出しパラメータのスタックメモリにコピーされます。
 * 移動後は、 もとの関数内の変数は使用できなくなります

*/
fn main() {
    let foo = Foo { x: 42 };

    // foo の所有権は do_something に移動
    do_something(foo);
    // foo は使えなくなる。下記のように `value borrowed here after move`と出る
    //  println!("{}", foo.x);
    /*
        say3no@pasokun src % rustc tour46_move_ownership.rs
    error[E0382]: borrow of moved value: `foo`
      --> tour46_move_ownership.rs:24:20
       |
    19 |     let foo = Foo { x: 42 };
       |         --- move occurs because `foo` has type `Foo`, which does not implement the `Copy` trait
    ...
    22 |     do_something(foo);
       |                  --- value moved here
    23 |     // foo は使えなくなる
    24 |     println!("{}", foo.x);
       |                    ^^^^^ value borrowed here after move

    error: aborting due to previous error

    For more information about this error, try `rustc --explain E0382`.
    say3no@pasokun src %

        */
}
