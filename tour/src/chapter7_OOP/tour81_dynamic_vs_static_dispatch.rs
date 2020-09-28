struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

fn static_make_noise(creature: &SeaCreature) {
    // we know the real type
    creature.make_noise();
}

// &dyn
fn dynamic_make_noise(noise_maker: &dyn NoiseMaker) {
    // we don't know the real type
    noise_maker.make_noise();
}

/*

dynamic vs static dispatch

メソッドは２つの方法で実行されます。

* static dispatch - インスタンスの型がわかっている場合、どの関数を呼び出すかを直接知ることができます。
* dynamic dispatch - インスタンスの型がわからない場合、正しい関数を呼び出す方法を見つけなければなりません

*/
fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    static_make_noise(&creature);
    dynamic_make_noise(&creature);
}
