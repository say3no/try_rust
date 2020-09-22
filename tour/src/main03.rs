#![allow(dead_code)] // この行でコンパイラのwarning メッセージを止めます

// Tour 23
struct SeaCreature {
    // String は構造体である
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

// Tour 27
// tuple likeな　sturctでは ; が必要
struct Location(i32, i32);

// Tour 28
struct Marker; // Unit like

// Tour 29
#[derive(Debug)] // kore nani
enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}

// Tour 30
enum PoisonType {
    Acidic,
    Painful,
    Lethal,
}

enum Size {
    Big,
    Small,
}

// Rust の enum は tagged-union とも言われています。
// 複数の型を組み合わせて新しい型を作ることができます。これがRustにはalgebraic typesを持つと言われる理由です
enum Weapon {
    Claw(i32, Size), // tuple like struct
    Poison(PoisonType),
    None,
}

fn main() -> () {
    // Tour 24
    // スタティックメソッドでStringインスタンスを作成する
    // StaticMethodは　:: で呼べる
    let s = String::from("Hello world!");
    // インスタンスを使ってメソッドを呼び出す
    // InstanceMethodは　.  で呼べる
    println!("{} is {} characters long.", s, s.len());

    // Tour 25 Memory
    // Rustはデータを肘するために次の三種類のメモリ空間を持っています
    // 1. Data Memory
    // 固定長もしくはスタティックなデータ。
    // たとえばHello worldみたいなプログラム内の文字列は読み取りにしか使えないため、この領域に入る。
    // コンパイラはこういったデータに対してチューニングしており、
    // メモリ上のいちはすでに知られていてかつ固定であるため、
    // 非常に早く使うことができる。

    // 2. Stack Memory
    // 関数内で定義された変数。関数が呼び出されている間は、
    // メモリ上の位置は変更されることがないため、
    // コンパイラからするとチューニングできるので、
    // スタティックメモリも非常に早くデータにアクセスできます。

    // 3. Heap Memory
    // プログラム実行時に作られるデータ。
    // このメモリにあるデータは追加、移動、削除、サイズの調節などの操作が許可されています。
    // 動的であるため、遅いと思われがちですが、これによりメモリの使い方に柔軟性を生み出すことができます。
    // データをヒープメモリに入れることをアロケーションといい、データをヒープメモリから削除することはディアロケーションといいます。

    /*
    // Tour 26
    // SeaCreatureのデータはスタックに入ります
    let ferris = SeaCreature { //
        // String 構造体もスタックに入りますが、
        // ヒープに入るデータの参照アドレスが一つ入ります
        animal_type: String::from("crab"), // "crab" はデータメモリ, String::from() はヒープ
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
        species: Species::Clam
    };

    let srah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("none"),
        species: Species::Octopus
    };

    println!("{} is a {}. They have {} arms, {} legs, and a {} weapon", ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon);
    println!("{} is a {}. They have {} arms, {} legs, and a {} weapon", srah.name, srah.animal_type, srah.arms, srah.legs, srah.weapon);

    // Tour 27
    // これもスタックに入れられる構造体
    let loc = Location(42,43);
    println!("{}, {}", loc.0, loc.1);

    // Tour 28
    let _m = Marker; // ()

    // Tour 29
    match ferris.species {
        Species::Crab => println!("{} is crab", ferris.name),
        Species::Octopus => println!("{} is octopus", ferris.name),
        Species::Fish => println!("{} is fish", ferris.name),
        Species::Clam => println!("{} is clam", ferris.name),
    }
    */

    // Tour 30
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => match ferris.weapon {
            Weapon::Claw(num_claws, size) => {
                let size_description = match size {
                    Size::Big => "big",
                    Size::Small => "small",
                };
                println!(
                    "ferris is a crab with {} {} claws",
                    num_claws, size_description
                )
            }
            _ => println!("ferris is acrab with some other wepons"),
        },
        _ => println!("ferris is some other animal"),
    }
}
