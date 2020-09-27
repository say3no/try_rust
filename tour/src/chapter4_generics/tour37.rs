fn do_somethin_taht_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

// main は 値を返さないが、エラーを返すことはある
fn main() -> Result<(), String> {
    let result = do_somethin_taht_might_fail(12);

    // match は Result をエレガントに分解して、
    // すべてのケースが処理されることを保証できます
    match result {
        Ok(v) => println!("発見 {}", v),
        Err(_e) => {
            // エラーをうまく処理

            // 何が起きたのかを説明する新しい Err を main から返します
            return Err(String::from("main でなにか問題が置きました"));
        }
    }

    Ok(())
}
