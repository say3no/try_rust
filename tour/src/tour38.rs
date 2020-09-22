fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値じゃないよ"))
    }
}

fn main() -> Result<(), String> {
    // コードが簡潔なのに注目
    let v = do_something_that_might_fail(2)?;
    // 上記は、下記と等価
    /*
    match do_something_that_might_fail() {
        Ok(v) => v,
        Err(e) => return Err(e);
    }
    */

    println!("発見 {}", v);
    Ok(())
}
