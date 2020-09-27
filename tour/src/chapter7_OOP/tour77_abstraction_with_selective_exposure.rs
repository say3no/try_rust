struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

/*
選択的露光による抽象化

Rustはオブジェクトの内部構造を隠すことができます。
デフォルトでは、フィールドやメソッドは、それらが属するモジュールにのみアクセス可能です。
pub キーワードは、モジュールの外部で構造体のフィールドやメソッドを公開します。
*/

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    println!("{}", creature.get_sound());
}
