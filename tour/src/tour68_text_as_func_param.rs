fn say_it_loud(msg: &str) {
    println!("{} !!!", msg.to_string().to_uppercase());
}

fn main() {
    // say_it_loud can borrow &'static str as &str
    say_it_loud("hello");
    // say_it_loud can borrow String as &str
    say_it_loud(&String::from("hello"))
}
