struct Foo {
    value: i32,
}

/**
* The . Operator
* The . operator is used in accessing fields and methods of a reference. It works a bit more subtly.
*
* Whoa, why didn't we need to add *** before ref_ref_ref_f ?
* This is because the . operator automatically dereferences a sequence of references.
* That last line is turend int o the following by the compiler automatically.
   println!("{}", (***ref_ref_ref_f).value);
*/
fn main() {
    let f = Foo { value: 42 };
    let ref_ref_ref_f = &&&f;
    println!("{}", f.value);
    println!("{}", ref_ref_ref_f.value);
    println!("{}", (***ref_ref_ref_f).value);
}
