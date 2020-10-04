use std::alloc::{alloc, Layout};
use std::ops::Deref;

struct Pie {
    secret_recipe: usize,
}

impl Pie {
    fn new() -> Self {
        // let's ask for 4 bytes
        let layout = Layout::from_size_align(4, 1).unwrap();

        unsafe {
            // allocate and save the memory location as a number
            let ptr = alloc(layout) as *mut u8;
            // use pointer math and write a few
            // u8 values to memory
            ptr.write(86);
            ptr.add(1).write(14);
            ptr.add(2).write(73);
            ptr.add(3).write(64);

            Pie {
                secret_recipe: ptr as usize,
            }
        }
    }
}

impl Deref for Pie {
    type Target = f32;
    fn deref(&self) -> &f32 {
        // interpret secret_recipe pointer as a f32 raw pointer
        let pointer = self.secret_recipe as *const f32;
        // dereference it into a return value &f32
        unsafe { &*pointer }
    }
}

/**
 * Familiar Friends
 *
 * Vec<T> や String のようなのようなスマートポインタについて考えてみましょう
 *
 * Vec<T>は、バイトのメモリ領域を持つスマートポインタです。Rustコンパイラはこれらのバイオtの中に何が存在するのかを知りません。
 * スマートポインタは、管理しているメモリ領域からアイテムを所得する意味を解釈し、
 * それらのバイト内のデータ構造がどこで始まり、どこで終わるかを追跡し、
 * 最終的に生のポインタをデータ構造にdereferencingして、人間工学に基づいたすっきりとしたインターフェイスに変換します。
 * (例: my_vec[3])
 *
 * 同様に、Stringはバイト単位のメモリ領域を追跡し、
 * そこに書き込まれた内容が常に有効なutf-8であるようにプログラム的に制限し、そのメモリ領域を&str型に派生させるのを助けます。
 *
 * これらのデータ構造は、どちらも生のぽいんたの安全でない参照解除を使用して仕事をしています
 *
 * メモリの詳細:
 *
 * RustにはC言語のmallocに相当するものがあり、allocとLayoutを使用して自分のメモリ領域を管理しています。
 */
fn main() {
    let p = Pie::new();
    // "make a pie" by dereferencing our
    // Pie struct smart pointer
    println!("{:?}", *p);
}
