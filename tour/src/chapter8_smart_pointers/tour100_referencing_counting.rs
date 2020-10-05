use std::rc::Rc;

struct Pie;

impl Pie {
    fn eat(&self) {
        println!("tasets better on the heap")
    }
}

/**
 * Referencing Ccounting
 *
 * RcはStackからHeap上にデータを移動させるスマートポインタです。
 * これにより、ヒープ上に置かれたデータを
 * 不変に借用する機能を持つRCスマートポインタを複製することができます。
 *
 * 最後のスマートポインタがドロップされたときだけ、ヒープ上のデータが開放されます
 */
fn main() {
    let heap_pie = Rc::new(Pie);
    let heap_pie2 = heap_pie.clone();
    let heap_pie3 = heap_pie2.clone();

    heap_pie3.eat();
    heap_pie2.eat();
    heap_pie.eat();
}
