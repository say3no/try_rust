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

    fn hello_world(&self) {
        println!("Hello World");
    }
}

/*
Polymorphism With Traits

Rust は traits を使った多相性をサポートしています。
Traits を 使用すると、
一連のメソッドを構造体の方に関連付けることができます。

最初に trait のメソッドのシグネチャを定義します。こんなかんじ

```
trait MyTrait {
    fn foo(&self);
    ...
}
```

構造体が trait を実装すると、実際の型を知らなくても、
Trait型(&dyn MyTraitなど) を使用して間接的に構造体と
対話できるようにする契約が確立されます。

構造体の実装された traits メソッドは、実装ブロック内で定義されます。
*/
impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        // over ride
        println!("{}", &self.get_sound());
    }
}

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    creature.make_noise();
    creature.hello_world();
}
