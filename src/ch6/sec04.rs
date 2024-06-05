// パーサージェネレーターでミニ言語を作ろう
// ダミーのmain関数
fn main() {}

/*
パーサージェネレーターとは、
文法を定義しておくことで、その定義に従ってコンパイラを生成するツール
コンパイラのコンパイラ(コンパイラコンパイラ)とも呼ばれる

Rustでは、
nom, lalrpop, combine, pegなどのパーサージェネレーターがある

ミニ言語を実装することを目標にして、自由度の高いpegを使用する
*/

// ルールの書き方
/*［書式］pegクレートで文法規則の指定
rule ルール名() -> 戻り値
    = 構文定義1 {アクション1}
    / 構文定義2 {アクション2}
    / 構文定義3 {アクション3}

pub ruleとすると、その規則がモジュール外に公開される
*/

// 簡単な計算プリを作る
// peg_plus

// 以下に文法規則を記述
peg::parser!( grammar plus() for str {
    // ルートとなる規則を追加
    pub rule eval() -> i64 // 規則の名前をevalにする
        = term() // 構文定義

    // 足し算を行う構文規則を追加
    rule term() -> i64 // 規則の名前
        = v1:num() "+" v2:num() // 構文定義
        { v1 + v2 } // アクション

    // 数値を読む規則を追加
    rule num() -> i64 // 規則の名前
        = values:$(['0'..='9']+) // 構文定義 正規表現[0-9]と似たような形
        { values.parse().unwrap() } // アクション parseメソッドでi64に変換子ている
});

#[test]
fn peg_plus_main() {
    // 足し算の計算式を実行
    println!("2+5={}", plus::eval("2+5").unwrap());
    println!("8+2={}", plus::eval("8+2").unwrap());
    println!("200+50={}", plus::eval("200+50").unwrap());
}

// 四則演算の計算アプリを作る
// peg_calc
peg::parser!( grammar calc() for str {
    // ルートとなる規則を追加
    pub rule eval() -> i64 // 規則の名前をevalにする
        = expr() // 構文定義

    // 足し算と引き算を行う構文規則を追加
    rule expr() -> i64 // 規則の名前
        // 複数の規則を指定
        // 演算子の優先順位(*/のほうが優先)のために*/はtermで指定し
        // termを呼び出すことで、*/を先に処理するようにしている
        // r.exprとしているのは、再帰的にexprを呼び出すため
        // r:exprが式の右側に来ているのは、左再帰(再帰下降構文解析で無限ループになる)を避けるため
        = l:term() "+" r:expr() {l + r}
        / l:term() "-" r:expr() {l - r}
        / term()

    // 掛け算とわり算を行う構文規則を追加
    rule term() -> i64 // 規則の名前
        // 丸括弧(式)は*/より優先度が高いので、exprのとき同様にvalueで指定している
        = l:value() "*" r:term() {l * r}
        / l:value() "/" r:term() {l / r}
        / v:value()

    // 数値を読む規則を追加
    rule value() -> i64 // 規則の名前
        = number() // 数値
        // v:expr()とすることで、()がネストしていても処理できる
        / "(" v:expr() ")" {v} // ()で囲まれた式

    // 数値を取得するルールを指定
    rule number() -> i64 // 規則の名前
        = n:$(['0'..='9']+) // 構文定義 正規表現[0-9]と似たような形
        { n.parse().unwrap() } // アクション parseメソッドでi64に変換子ている

});

#[test]
fn peg_calc_main() {
    // 足し算の計算式を実行
    println!("{}", calc::eval("1+2*3").unwrap());
    println!("{}", calc::eval("(1+2)*3").unwrap());
    println!("{}", calc::eval("3*(1+2)").unwrap());
    println!("{}", calc::eval("100/2-1").unwrap());
}

// ミニ言語を作る!
// peg_tomato

mod peg_tomato;
use peg_tomato::node;
use peg_tomato::parser;
use peg_tomato::runner;
use std::fs;

#[test]
fn peg_tomato_main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] peg_tomato file.tomato");
        return;
    }
    // ファイルを開く
    // let filename = &args[1];
    let filename = "./sample-src/ch6/peg_tomato/sum1to10.tomato";
    // let filename = "./sample-src/ch6/peg_tomato/fib.tomato";
    // let filename = "./sample-src/ch6/peg_tomato/fizbuzz.tomato";
    let src = fs::read_to_string(filename).unwrap();
    runner::run(&src);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        assert_eq!(runner::run("print 32"), 32);
        assert_eq!(runner::run("print 1+2*3"), 7);
        assert_eq!(runner::run("a=3;if a==3 { print 1 } else { print 0 }"), 1);
        assert_eq!(runner::run("a=0;for i=1 to 10 { a=a+i }; print a"), 55);
        assert_eq!(runner::run("print \"abc\""), 0);
    }
}
