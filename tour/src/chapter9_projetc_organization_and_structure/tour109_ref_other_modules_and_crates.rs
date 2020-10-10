use std::f64::consts::PI;

/**
 * Referencing Other Modules and Crates
 *
 * モジュール内のアイテムは、モジュールのフルパス `std::f64::consts::PI` で参照することができます。
 *
 * もっと簡単な方法は use キーワードです。
 * これを使用すると、フルパスでなくてもコード全体で
 * 使用したいモジュールの特定の項目を指定することができます。
 * 例えば use std::f64::consts::PI を使うと、メイン関数で識別子 PI を使う事ができます。
 *
 * std は Rustの標準ライブラリのクレートであり、有用なデータ構造やオペレーティング・システムと対話するための関数が満載です。
 *
 * コミュニティによって作成されたクレートの検索可能なディレクトリは https:crates.io にあります。
 */
fn main() {
    println!("Welcome to the playground!");
    println!("I would love a slice of {}!", PI);
    println!("{}", std::f64::consts::E);
}
