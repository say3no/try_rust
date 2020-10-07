use std::sync::Mutex;

struct Pie;

impl Pie {
    fn eat(&self) {
        println!("only I eat the pie right now!");
    }
}

/**
 * Sharing Across Threads
 *
 * Mutexは、Smart Pointerが一般的に保持しているコンテナデータ構造で、
 * データを取り込み、その中のデータへのmutable/immutableな参照を借用することができます。
 * これは、オペレーティング・システムが一度に一つのCPUスレッドだけがデータにアクセスできるように制限し、
 * その元のスレッドがロックされた借用が完了するまで、他のスレッドをブロックすることで、借用が悪用されるのを防ぎます。
 *
 * マルチスレッドは Tour of Rust の範囲を超えていますが、Mutexは複数のCPUスレッドが同じデータにアクセスする際の基本的な部分です。
 *
 * 特別なスマートポインタ Arc がありますが、これは参照カウントのスレッドセーフなインクリメントをする以外はRcと同じです。
 * 同じMutexへの参照が多数ある場合によく使われます
 *
 */

fn main() {
    let mutex_pie = Mutex::new(Pie);
    // let's borrow a locked immutable reference of pie
    // we have to unwrap the result of a lock
    // because it might fail
    let ref_pie = mutex_pie.lock().unwrap();
    ref_pie.eat();
    // locked referenec drops here, and mutex protected value can be used by someone else
}
