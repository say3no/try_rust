/*
Chars

Unicodeでの作業が非常に困難なため、
Rustではutf-8バイトのシーケンスを char 型の 文字のベクトルとして取得する方法が提供されています。

char は常に4byteの長さです。(これによって個々の文字を効率的に検索できるようになっています)
*/

fn main() {
    // 文字を char のベクトルとして集める
    let chars = "hello 👾".chars().collect::<Vec<char>>(); // collect 型ってのがある？
    println!("{}", chars.len());

    // chars は 4バイトなので、 u32 に変換することができる
    println!("{}", chars[3] as u32);
}
