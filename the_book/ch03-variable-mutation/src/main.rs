fn main() {
    let mut x = 5;
    println!("The value of x is : {}", x); // x の値は{} です
    x = 6; // comiple error, can not borrow x, because x is not mutable variable
    println!("The value of x is : {}", x); // x の値は{} です
    let y = 6;
    println!("The value of y is : {}", y); // x の値は{} です
    let y = 7; // shadowing, conceal
    println!("The value of y is : {}", y); // x の値は{} です
    let y = y + 1; // shadowing
    println!("The value of y is : {}", y); // x の値は{} です
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is : {}", spaces); // x の値は{} です

    // # Scalar Type
    // ##  Int, Float, Boolean, Character

    // ### Int
    // i8, u8, i16, u16, i32, u32, i64, u64
    // isize usize
    // 整数型の基準はi32型です:
    // 64ビットシステム上でも、 この型が普通最速になります。
    //  isizeとusizeを使う主な状況は、何らかのコレクションにアクセスすることです。

    // ## Float
    // f32, f64

    // ## Boolan
    // true, false

    // ## Character Type
    // charはシングルクォートで指定すること。
    let c = 'z';
    println!(" c is {}", c);

    // # Compound Types(複合型)
    // Tuple, Array

    // ## Tuple
    // 複数の型を許容する
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tupleの値にはこういうアクセスができる
    println!(" x, y, z: {},{},{} ", tup.0, tup.1, tup.2);

    // こういうのもできる
    let (x, y, z) = tup;
    println!(" x, y, z: {},{},{} ", x, y, z);
    let (x, y, z) = (123, 0.1, 4);
    println!(" x, y, z: {},{},{} ", x, y, z);

    // ## Array
    // 単一の方を複数個もつのがArray
    // VectorなどのCollection型ほど協力ではない、原始的ゆれに強力な型、それがArray
    let arr = [1, 2, 3, 4];

    println!(" 1, 2, 3: {},{},{} ", arr[1], arr[2], arr[3]);
    // 配列の終端を超えて要素にアクセスしようとすると、 **コンパイルは通るが実行するとエラーになる**(panicする)
    // panicとはプログラムがエラーで終了したことを表すRust用語

    // Rustでは、メモリアクセスを許可し、処理を継続する代わりに即座にプログラムを終了することで、 この種のエラーからプログラマを保護しています。

    // # Functions
    another_functions(5);

    // ## 関数本体は、文(Statements)と式(Expressions)を含む
    // 関数本体は、文が並び、最後に敷きを置くか文を置くという形で形成されます。

    // 文とは、何らかの動作をして値を返さない命令です。
    // let y = 6;
    // これは、文。値を返さない
    // 代入が代入値を返す言語も存在するが、Rustの場合は返さない
    // statement_function()という関数をかいた。何も実行していない。これも、文。
    statement_function();

    // 式は結果値に評価されます。
    // 式はなにかに評価され、これからあなたが書くRustコードの多くを構成します。
    // 5+6は、11に評価される式。
    // 式は、文の一部になりえます。
    // たとえば、 let y = 6はという文の値6は、値6に評価される式です。
    // 関数呼び出しも, {}も式。

    // ## 戻り値の関数
    expression_function();

    let x = expression_function();
    println!("x is {} ", x);

    loop {
        println!("again!");
        break;
    }

    let mut number = 5;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let mut index = 0;

    while index < 3 {
        println!("the value is : {}", arr[index]);
        index = index + 1;
    }

    for ele in arr.iter() {
        println!("{}", ele);
    }

    for a in (1..4).rev() {
        println!("{}!", a);
    }
}

fn another_functions(x: i32) {
    println!("Another function. {}", x);
}

fn statement_function() {}

fn expression_function() -> i32 {
    5 // セミコロンなしなので、式であり、返り値をもつ
      // セミコロンをつけた場合、文なので帰り値をまたないため、コンパイルエラーとなる
}
