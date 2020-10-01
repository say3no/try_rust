/*
References can be converted into a more primitive type called a raw pointer.
Much like a number, it can be copied and moved around with little restriction.
Rust makes no assurances of the validity of memory location it points to.

参照は、生のポインタと呼ばれる、より原始的な型に変換することができます。
数値のように、ほとんど再現なくコピーしたり移動したりすることができます。
Rust は、ポインタが指すメモリの位置の妥当性を保証しません。

Two kinds of raw pointers exist:
二種類の生ポインタが存在します。

* *const T - A raw pointer to data of type T that should never change.
決して変更されるべきではないT型のデータへの生ポインタ

* *mut T - A raw pointer to data of type T that can change.
変更可能なT型のデータへの生ポインタ

Raw pointers can be converted to and from numbers(e.g. usize)
生ポインタは、数値に変換したり、数値から変換したりすることができます(例: usize)

生ポインタは、安全でないコードでデータにアクセスすることができます(これについては後述します)

メモリの詳細
* Rustにおける参照は、使用方法の点ではC言語のポインタに非常に似ていますが、
  保存方法や他の関数への移動方法についてはコンパイル時の制限があります。

* Rustの生ポインタは C言語のポインタに似ていますが、
  コピーしたり渡したりできる数値を表し、
  数値型に変換してポインタの計算を行うために数値として修正することもできます。
*/
fn main() {
    let a = 42;
    let memory_location = &a as *const i32 as usize;
    println!("Data is here {} ", memory_location);
}
