fn main() {
    let s1 = gives_ownership(); // gives_ownership は, 戻り値をs1にムーブ

    let s2 = String::from("hello"); // s2がスコープに入る

    let s3 = takes_and_gives_back(s2);
    // s2 は takes_and_gives_back にムーブされ、
    // 戻り値も s3 にムーブされる

    let s1 = String::from("oisu");
    let (s2, len) = calculate_length(s1);
    // 何かをMoveさせてこねこねして、なにかプラスアルファが欲しい場合、タプルを使えばできるけど、全部にやるのは大変
    println!("The lengh of {} is {}.", s2, len);

    // そういうときは変数本体ではなく、その参照だけを渡してあげる
    let s1 = String::from("nyanpassssssss");
    let len = calculate_length2(&s1); // 参照するというこは、所有権を移すということではない。
    println!("The lengh of {} is {}.", &s1, len);

    // 関数の引数に参照を取ることを借用と呼びます。
    let mut s = String::from("oisu");
    change(&mut s);
    println!("{}", s);

    // 可変な参照は大きな制約が一つある。
    // 特定のスコープで、ある特定のデータに対しては、一つしか可変な参照を持てない
    let r1 = &mut s;
    r1.push_str("aiueo kaikukeko dayo");
    println!("{}", r1);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

/*
fn change(some_string: &String) {
    // some_string.push_str(", world"); // 借りたもんに変更を加えるな。常識ねえのかよ
    1
}
*/

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 戻り値を呼び出し元へムーブする
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}
