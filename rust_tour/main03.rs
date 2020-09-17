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
    // Tour 24
    // スタティックメソッドでStringインスタンスを作成する
    // StaticMethodは　:: で呼べる
    let s = String::from("Hello world!");
    // インスタンスを使ってメソッドを呼び出す
    // InstanceMethodは　.  で呼べる
    println!("{} is {} characters long.", s, s.len());



}