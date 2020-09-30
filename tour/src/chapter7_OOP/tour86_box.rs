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

struct Ocean {
    animals: Vec<Box<dyn NoiseMaker>>,
}

/*
Box

Box は stack memory から heap メモリへと移動させるためのデータ構造体です。

Boxは、ヒープ上のデータへのポインタを保持するスマートポインタとして知られる構造体です。

Boxはサイズが既知の構造体であるため(ポインタを保持しているだけなので)、
フィールドのサイズを知らなければならない構造体の中への参照をする方法としてよく使用されます。

Boxはどこからでも使用できるほど一般的です
*/

fn main() {
    let ferris = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    let sarah = SeaCreature {
        name: String::from("Sarah"),
        noise: String::from("swish"),
    };
    let ocean = Ocean {
        animals: vec![Box::new(ferris), Box::new(sarah)],
    };

    for a in ocean.animals.iter() {
        a.make_noise();
    }
}
