/*
文字列スライス

文字列スライスは、常に有効な utf-8 でなければならないメモリ内のバイト列への参照です。

文字列のスライス(サブスライス)である str のスライスも有効な　utf-8 でなければなりません。

&str の一般的なメソッド
* len は文字列リテラルの長さを ** バイト単位で** 取得します(文字数ではない)
* 基本的なテストのための start_with, ends_with
* is_emptyは長さがゼロのときにtrueを返します
* find はテキストの最初の位置の Option<usize> を返します
*/
fn main() {
    let a = "hi 👾";
    println!("{}", a.len());

    let first_word = &a[0..2];
    let second_word = &a[3..7];
    // let half_crab = &a[3..5]; は失敗します。
    // Rust は無効な unicode 文字のスライスを受け付けません
    println!("{} {}", first_word, second_word);
}
