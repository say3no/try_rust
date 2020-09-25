static PI: f64 = 3.1415;

/*
スタティックライフタイム

スタティック変数は、コンパイル時に作成され、プログラムの *開始から終了まで* 存在するメモリリソースです。
これらの変数の型は明示的に指定しなければなりません。

スタティックライフタイムは、プログラムの終了まで *無期限に* 持続するメモリリソースです。
この定義では、スタティックライフタイムを持つリソースは実行時にも作成できることに注意してください。

スタティックライフタイムを持つリソースには、特別なライフタイム指定子 'static があります。

'static リソースは ** けっしてドロップすることはありません**

スタティックライフタイムを持つリソースが参照を含む場合、それらはすべて 'static でなければなりません＊そうでなければ、参照はプログラムの終了前にドロップｐする可能性があります。

メモリの詳細:
 * スタティック変数を変更することは本質的に危険です。なぜならスタティック変数は誰でもグローバルにアクセスして読み取ることができるからです。
 * グローバルデータの課題については後ほど説明します。

 * Rustではコンパイラがメモリを保証できない操作を実現するために、unsafe {...} ブロックを使用することができます。
 * rustnomiconについて気軽に話してはいけません。


*/
fn main() {
    // スタティック変数は関数スコープでも定義可能
    static mut SECRET: &'static str = "swordfish";

    // 文字列リテラルは 'static ライフタイム
    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    //ルールを破ることはできますが、それを明示する必要があります。
    unsafe {
        // 文字列リテラルは 'static なので、 SECRET に代入可能
        SECRET = "abracadabra";
        println!("{}", SECRET);
    }
}
