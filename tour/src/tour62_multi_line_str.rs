fn main() {
    let haiku: &'static str = "
    書いてみたり
    消したりはては
    ケシの花
    - 立花北枝";

    println!("{}", haiku);

    println!(
        "こんにちは \
    世界"
    ); // 世界の前にある間隔は無視されます
}
