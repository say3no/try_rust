use std::cell::RefCell;
use std::rc::Rc;

struct Pie {
    slices: u8,
}

impl Pie {
    fn eat_slice(&mut self, name: &str) {
        println!("{} took a slice!", name);
        self.slices -= 1;
    }
}

struct SeaCreature {
    name: String,
    pie: Rc<RefCell<Pie>>,
}

impl SeaCreature {
    fn eat(&self) {
        // use smart pointer to pie for a mutable borrow
        let mut p = self.pie.borrow_mut();
        // take a bite !
        p.eat_slice(&self.name);
    }
}

/**
 * Combining Smart Pointers
 *
 * スマートポインタは限定的に見えるかもしれませんが、非常に強力な組み合わせを作ることができます。
 *
 * Rc<Vec<Foo>> - ヒープ上の不変データ構造体の同じベクトルを借りることができる複数のスマートポインタの複製を許可します
 * Rc<RefCell<Foo>> - 複数のスマートポインタが、同じ構造体Fooをmutable/immutableに借りることができるようにします
 * Arc<Mutex<Foo>> - 複数のスマートポインタに、CPUスレッド排他的な方法で、一時的なmutable/immutableな借用をロックする機能を許可します
 *
 * メモリの詳細
 *
 * これらの組み合わせの多くにテーマがあることに気づくでしょう。
 * 内部データを変更するための不変データ型(複数のスマートポインタが所有している可能性がある)
 */
fn main() {
    let pie = Rc::new(RefCell::new(Pie { slices: 8 }));
    // ferris and sarah are given clones of smart pointer to pie
    let ferris = SeaCreature {
        name: String::from("ferris"),
        pie: pie.clone(),
    };
    let sarah = SeaCreature {
        name: String::from("sarah"),
        pie: pie.clone(),
    };
    ferris.eat();
    sarah.eat();

    let p = pie.borrow();
    println!("{} slices left", p.slices);
}
