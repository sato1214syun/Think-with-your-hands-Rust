// RustでBMI肥満度判定ツール
use std::io;

fn main() {
    // 身長と体重の入力
    let height = input("身長(cm)は? ") / 100.0;
    let weight = input("体重(kg)は? ");
    // BMIの計算
    let bmi = weight / height.powf(2.0); // powfは浮動小数点の二乗
    println!("BMI={:.1}", bmi);
    // 肥満度判定
    match bmi {
        x if x < 18.5 => println!("低体重"),
        x if x < 25.0 => println!("普通体重"),
        x if x < 30.0 => println!("肥満度1度"),
        x if x < 35.0 => println!("肥満度2度"),
        x if x < 40.0 => println!("肥満度3度"),
        _ => println!("肥満度4度"),
    }
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("入力エラー"); // 標準入力から1行読み込み
    s.trim().parse().expect("数値変換エラー")  // parseで文字列を数値(f64)に変換
}
