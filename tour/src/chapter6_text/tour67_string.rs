/*
A String is as struct that wons a sequence of utf-8 bytes in heap meoary.

Because sits memory is on the heap, it can be extended, modified, etc. in ways string literals cannnot.

Common methods:
* push_str to add more uf08 bytes to the end of string
* replace to replace sequence of utf-8 bytes with others
* to_lowercase/to_uppercase for case changes
* trim for trimming space

When a String is dropped ,its heapmemory is also dropped;

String has a + operator that extends the string with a &str and returns itself,
 but it might not be as ergonomic as you hope for.0w:9
*/

fn main() {
    let mut helloworld = String::from("hello");
    helloworld.push_str(" world");
    helloworld = helloworld + "!!";
    println!("{}", helloworld);
}
