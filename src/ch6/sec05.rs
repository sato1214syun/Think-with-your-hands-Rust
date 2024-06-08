// C言語やPythonとの連携
// ダミーのmain関数
fn main() {}

// ffiでC言語を連携
// C言語のライブラリーを作る

// ffi_mul内にC言語で書いたファイルmycalc.cを用意

// C言語を使用するRustのメインプログラム
// ffi_mul/main.rs

// C言語で書いた里ぶらりの名前を指定
// lint()でライブラリ名と、ライブラリの種類を指定
#[link(name = "mycalc", kind = "static")]
extern "C" {
    // C言語で定義した関数を指定
    fn mul(a: isize, b: isize) -> isize;
}

#[test]
fn ffi_mul_main() {
    // C言語の関数を呼び出す
    unsafe {
        // unsafeで実行する必要がある
        let n = mul(30, 5);
        println!("{}", n);
        let n = mul(8, 80);
        println!("{}", n);
    }
}

// ライブラリをRustで作って、C言語で呼び出す
// mycalc_rs_testディレクトリを参照

// RustでC言語のデータ型を扱うヒント
// RustでC言語の基本型を扱う → std::os::rawモジュールを使う

// C言語のポインタをRustで扱う → *constのような型を指定するとRustの参照ポインタ型に代入できる
// pointer_test.rs
#[test]
fn pointer_test_main() {
    let val: i32 = 10;
    let val_ptr: *const i32 = &val;
    println!("val = {}\nval_ptr = {:?}", val, val_ptr);
}

// C言語の文字列を扱う → std::ffi::{CString, CStr}を使う
// CStringは所有権システムにより、C言語の文字列と違ってスコープを抜けると破棄されることに注意
// cstr_test.rs
use std::ffi::CString;
#[test]
fn cstr_test_fn() {
    // CStringの生成
    let msg_cstr = CString::new("こんにちは。").unwrap();
    // C言語のライブラリを呼び出す
    unsafe {
        // c_lang_lib::print_str(msg_cstr.as_ptr());
    }
}

// 動的ライブラリを作成してPythonから使う
// 省略

// PyO3を使うと便利