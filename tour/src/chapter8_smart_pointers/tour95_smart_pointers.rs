use std::ops::Deref;

struct TattleTell<T> {
    value: T,
}

impl<T> Deref for TattleTell<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("{} was used!", std::any::type_name::<T>());
        &self.value
    }
}

/***
 * Smart Pointers
 *
 * Rustでは、 &演算子を使って既存の型付きデータへの参照を作成する機能に加えて、
 * SmartPointerと呼ばれる参照のような構造体を作成する機能を提供しています。
 *
 * 高レベルでの参照は、別の型へのアクセスを与えてくれる型と考えることができます。
 * スマートポインタは通常の参照とは動作が異なり、プログラマが書く内部ロジックに基づいて動作します。
 * プログラマであるあなたがスマートな部分です。
 *
 * 通常、スマートポインタは Deref, DerefMut, Drop の各特性を実装しており、
 * 構造体が　* および . 演算子で参照解除されたときに何が起こるかのロジックを指定します。
 */
fn main() {
    let foo = TattleTell {
        value: "Secret Message",
    };
    // dereference occurs here immediately
    // after foo is auto-referenced for the
    // function `len`

    // .len() は何らかの impl を参照していることになる。
    // それを deref(*や.) するときに、 Deref.deref が呼ばれることになっている(のだと思う)
    println!("{}", foo.len());
}
