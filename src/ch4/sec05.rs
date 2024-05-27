// 列挙型とパターンマッチング
// ダミーのmain
fn main() {}

// null安全な言語Rust
// counter_fix.rs
struct Counter {
    value: i64,
}
impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }
    fn inc(&mut self) {
        self.value += 1;
        println!("value={}", self.value);
    }
}

// Counter構造体を引数にとる関数 --- (*2)
fn count(counter: Option<&mut Counter>) {
    match counter {
        None => return,
        Some(c) => c.inc(),
    };
}
#[test]
fn counter_fix_main() {
    // Counterオブジェクトを引数に呼ぶ --- (*3)
    let mut a = Counter::new();
    count(Some(&mut a));
    count(Some(&mut a));
    // Noneオブジェクトを引数に呼ぶ --- (*4)
    let a = None;
    count(a);
}

// 列挙型を定義する
// coin.rs
// 硬貨の種類を表す列挙型 --- (*1)
enum Coin {
    Coin1(isize),
    Coin5(isize),
    Coin10(isize),
    Coin50(isize),
    Coin100(isize),
    Coin500(isize),
}
impl Coin {
    // 硬貨の種類から実際の金額を計算 --- (*2)
    fn calc_price(&self) -> isize {
        match *self {
            Coin::Coin1(v) => v,
            Coin::Coin5(v) => v * 5,
            Coin::Coin10(v) => v * 10,
            Coin::Coin50(v) => v * 50,
            Coin::Coin100(v) => v * 100,
            Coin::Coin500(v) => v * 500,
        }
    }
}
#[test]
fn coin_main() {
    // 財布の中にある硬貨の種類と枚数を指定 --- (*3)
    let wallet: Vec<Coin> = vec![
        Coin::Coin1(3), // 1円が3枚
        Coin::Coin5(10), // 5円が10枚
        Coin::Coin10(5), // 10円がが5枚
        Coin::Coin50(1), // 50円が1枚
        Coin::Coin100(1), // 100円が1枚
        Coin::Coin500(5), // 500円が5枚
    ];
    // 金額を計算して表示 --- (*4)
    // foldはイテレータの各要素に対して(初期値, クロージャ)を適用する
    let total = wallet.iter()
        .fold(0, |sum, v| sum + v.calc_price());
    println!("財布の合計は{}円です", total);
}

// パターンマッチング
// match_num.rs
#[test]
fn match_num_main() {
    let i = 2u8;
    match i {
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("..."),
    }
}

// match_age.rs
#[test]
fn match_age_main() {
    let age = 8;
    let age_str = match age {
        // ".."や"..="で範囲を指定する
        0 => "乳児",
        1..=5 => "幼児",
        6..=11 => "こども",
        _ => "おとな",
    };
    println!("{}才は{}料金", age, age_str);
}

// match文でFizzBuzzを実装
// match_fizzbuzz.rs
#[test]
fn match_fizzbuzz_main() {
    for i in 1..=100 {
        // 値をmatchで分岐するがタプルを指定 --- (*1)
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"), // --- (*2)
            (0, _) => println!("Fizz"), // --- (*3)
            (_, 0) => println!("Buzz"),
            _      => println!("{}", i),
        }
    }
}

// マッチガードを使ってFizzBuzzを実装
// match_fizzbuzz_if.rs
#[test]
fn match_fizzbuzz_if_main() {
    for i in 1..=100 {
        let msg = match i {
            n if n % 15 == 0 => "FizzBuzz".to_string(),
            n if n % 3 == 0 => "Fizz".to_string(),
            n if n % 5 == 0 => "Buzz".to_string(),
            _ => format!("{}", i),
        };
        println!("{}", msg);
    }
}

// Optionなどの列挙型を使用しつつ、Someの値に応じたパターンマッチング
// match_bmi.rs
// BMIと肥満度表示する関数 --- (*1)
fn print_bmi(height: f32, weight: Option<f32>) {
    // weightに値があればBMIを求めてOption型で返す --- (*2)
    let bmi:Option<f32> = match weight {
        Some(w) => Some(w / (height / 100.0).powf(2.0)),
        None => None,
    };
    // BMIの値に応じて判定 --- (*3)
    let msg = match bmi {
        Some(n) if n < 18.5 => "低体重",
        Some(n) if n < 25.0 => "普通体重",
        Some(n) if n < 30.0 => "肥満1度",
        Some(n) if n < 35.0 => "肥満2度",
        Some(n) if n < 40.0 => "肥満3度",
        Some(_) => "肥満4度",
        None => "測定不能",
    };
    // 判定結果を表示 --- (*4)
    println!("BMI={:.1}, 判定={}", bmi.unwrap_or(0.0), msg);
}
// いろいろな入力を与える --- (*5)
#[test]
fn match_bmi_main() {
    let height = 162.3;
    print_bmi(height, Some(48.0));
    print_bmi(height, Some(72.3));
    print_bmi(height, None);
}





