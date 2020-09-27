struct SeaCreature {
    noise: String,
}

impl SeaCreature {
    fn get_sound(&self) -> &str {
        &self.noise
    }
}

/*
 Encapsulation With Methods

 Rustは、いくつかの関数(メソッドとも呼ばれる)に関連付けられた構造体であるオブジェクトの概念をサポートしています。
 メソッドの最初のパラメータは、メソッド呼び出しに関連付けられたインスタンスへの参照でなければなりません。

 * &self - インスタンスへの不変の参照
 * &mut self - インスタンスへ可変の参照
 メソッドはキーワード impl で実装ブロック内で定義されます。
*/
fn main() {
    let creature = SeaCreature {
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound());
}
