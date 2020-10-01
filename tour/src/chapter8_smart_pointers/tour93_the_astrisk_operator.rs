/**
 *  The * Operator
 * The * operator is an explicit way to dereference a reference
 *
 * Memory detail:
 *  * Because i32 is primitive type that implements the Copy trait,
 *    the bytes of variable a on stack are copeid into the bytesof variable b
 */
fn main() {
    let a: i32 = 42;
    let ref_ref_ref_a: &&&i32 = &&&a;
    let ref_a: &i32 = **ref_ref_ref_a; // de ref twice
    let b: i32 = *ref_a;
}
