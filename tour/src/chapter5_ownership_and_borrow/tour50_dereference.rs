/*
 &mut による参照では、 * 演算子によって参照を外す(derefereence)ことで、所有者の値を設定できます。
 * 演算子によって所有者の値のコピーを取得することもできます(コピー可能な型については後述)
*/
fn main() {
    let mut foo = 42;
    let f = &mut foo; // borrow as mut (write lock 取得 )

    // println!("{}", foo); 参照不可でコンパイルエラー
    println!("{}", f); // 42

    let bar = *f; //  所有者の値をコピー。値だけね。
    println!("{}", f); // 42
    println!("{}", bar); // 42
    *f = 13;
    println!("{}", f); // 13
    println!("{}", bar); // 42
    println!("{}", foo); // 13
}
