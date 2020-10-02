/**
 *  Smart Unsafe Code
 *
 * スマートポインタは安全でないコードをかなり頻繁に使用する傾向があります。
 * 前述したように、それらは Rust の最低レベルのメモリと対話するための一般的なツールです。
 *
 * 安全でないコードとは？
 * 安全でないコードは、Rust コンパイラが保証できないいくつかの機能を覗いて、通常のRustと同じように動作します。
 *
 * 安全でないコードの主な能力は、生のポインタを参照することです。
 * これは、メモリ上のある位置への生ポインタを取得し、"データ構造がここに存在する！"と宣言し、
 * それを使用可能なデータの表現に反感することを意味します。
 * (例えば *const u8 を u8 に変換)。 Rust には、メモリに書き込まれるすべてのバイトの意味を追跡する方法がありません。
 * Rust は 生のポインタとして使用される任意の数に何が存在するかを保証することができないため、参照の解除は unsafe block に格納されます
 *
 * スマートポインタは広範囲にrawポインタを参照しますが、その機能は十分に証明されています。
 *
 */
fn main() {
    let a: [u8; 4] = [86, 14, 73, 64];
    // this is a raw pointer. Getting the memory address
    // of something as a number is tottaly
    let pointer_a = &a as *const u8 as usize;
    // Turning our number into a raw pointer to a f32 is
    // aslo safe to do
    let pointer_b = pointer_a as *const f32;
    let b = unsafe {
        // this is unsafe because we are telling the compiler
        // to assume our pointer is a valid f32 and
        // dereference it's value into the variable b.
        //  Rust has no way to verify this assumption is true.
        *pointer_b
    };
    println!("I swear this is a pie! {}", b);
}
