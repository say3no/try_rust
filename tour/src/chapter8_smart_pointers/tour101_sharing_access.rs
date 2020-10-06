use std::cell::RefCell;

struct Pie {
    slices: u8,
}

impl Pie {
    fn eat(&mut self) {
        println!("tastes better on the heap!");
        self.slices -= 1;
    }
}

/**
 * Sharing Access
 *
 * RefCellは、スマートポインタによって一般的に保持されているコンテナデータ構造で、
 * データを取り込み、その中にあるものへのmutableな参照を借用することができます。
 * 内部のデータを借りようとしたときに、
 * 実行時にRustのメモリ安全ルールを適用することで、借用が悪用されるのを防ぎます。
 *
 * mutableな参照は一つだけ、あるいは複数のimutableな参照を使えますが、両方使用することはできません。
 *
 * これらのルールに違反すると、RefCellはパニックに陥ります
 */
fn main() {
    // RefCell validates memory safety at runtime
    // notice: pie_cell is not mut!
    let pie_cell = RefCell::new(Pie { slices: 8 });

    // シュッとスコープ作れる
    {
        // but we can borrow mutable references!
        let mut mut_ref_pie = pie_cell.borrow_mut();
        mut_ref_pie.eat();
        mut_ref_pie.eat();

        // mut_ref_pie is dropped at end of scope
    }

    // now we can borrow immutably once our mutable reference drops
    let ref_pie = pie_cell.borrow();
    println!("{} slices left", ref_pie.slices)
}
