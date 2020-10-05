struct Pie;

impl Pie {
    fn eat(&self) {
        println!("tastes better on the heap")
    }
}

/**
 * Heap Allocated Memory
 *
 * Boxは、StackからHeapにデータを移動させるためのスマートポインタです。
 * これを参照解除することで、ヒープに割り当てられたデータを
 * もとの方であるかのように人間工学的に使用することができます。
 */
fn main() {
    let heap_pie = Box::new(Pie);
    heap_pie.eat();
}
