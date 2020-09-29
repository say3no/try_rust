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

fn generic_make_noise<T>(creature: &T)
where
    T: NoiseMaker,
{
    // we know the real type at compile-time
    creature.make_noise();
}

/*
Generic Functions

Rust のジェネリックは形質と連携して動作します。
パラメータ化された型Tを記述する際に、引数が実装しなければならない形質をリストアップすることで、
どの型が引数として使用できるか制約することができます。

この例では、型T は Foo を実装しなければなりません。

```
fn my_function<T>(foo: T)
where
    T:Foo
{
    ...
}
```

ジェネリックを使用することで、コンパイル時に型とサイズが既知の静的型付き関数を作成することができ、
static dispatchを実行してサイズのある値として保存することができます。
*/

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    generic_make_noise(&creature);
}
