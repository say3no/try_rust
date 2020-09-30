// TODO: 動かない。よくわからん https://tourofrust.com/87_ja.html
struct MyStruct<T>
where
    T: MyTrait,
{
    piyo: T,
}

impl<T> MyStruct<T> {
    pub fn hoge() {
        println!("hogehoge");
    }
}

trait MyTrait {
    fn piyo();
}

impl MyTrait for MyStruct {
    fn piyo(&self) {
        println!("piyopiyo")
    }
}

/*
Generic Structs Revisited

Generic structs は Traits に束縛された struct みたいなもんです。

*/

fn main() {
    let a = MyStruct { foo: 2 };
    a.hoge();
}
