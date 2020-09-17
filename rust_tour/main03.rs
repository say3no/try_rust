// Tour 23
#[derive(Debug)]
struct SeaCreature {
    // String は構造体である
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() -> () {
    // スタティックメソッドでStringインスタンスを作成する
    let s = String::from("Hello world!");
    // インスタンスを使ってメソッドを呼び出す
    println!("{} is {} characters long.", s, s.len());
}