fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

/* unwap()は、下記と等価
match my_option {
    Some(v) => v,
    None => panic!("Rustによって生成されたエラーメッセージ")
}

or

match my_result {
    Ok(v) => v,
    Err(e) => panic!("Rustによって生成されたエラーメッセージ")
}

良いRust使い(Rustacean)であるためには、可能な限り適切に match を使用してください
いたずらに unwarp を使わずに きちんと match を書きましょう.あるいは ? を使おう
*/

fn main() -> Result<(), String> {
    // 簡潔ですが、値が存在することを仮定しており、
    // すぐにだめになる可能性があります
    let v = do_something_that_might_fail(42).unwrap();
    println!("発見 {}", v);

    // パニックするでしょう！
    let v = do_something_that_might_fail(1).unwrap();
    println!("発見 {}", v);

    Ok(())
}
