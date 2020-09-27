/*
文字列変換
多くの型は to_string を用いて文字列に変換することができます。
ジェネリック関数 parse を用いることで、文字列や文字列リテラルを型月の値に変換できます。
この関数は失敗する可能性があるので Resultを返します。
*/
fn main() -> Result<(), std::num::ParseIntError> {
    let a = 42;
    let a_string = a.to_string();
    let b = a_string.parse::<i32>()?;
    println!("{} {}", a, b);
    Ok(())
}
