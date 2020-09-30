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
        println!("{}", self.get_sound());
    }
}

fn generic_make_noise(creature: &impl NoiseMaker) {
    // we know real type at cmpile-time
    creature.make_noise();
}

/*
Generic Function Shorthand

Rust には、形質によって制約されたジェネリックを表現するための略語があります。

```
fn my_function(foo: impl Foo){
    ...
}
```

上記は、下記と同じ意味を持ちます

```
fn my_function<T>(foo: T)
where
    T:Foo
{
    ...
}
```
*/
fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    creature.make_noise();
    generic_make_noise(&creature);
}
