use std::fs;

fn main() {
    let a_file = "sample-src/ch2/fizzbuzz_python.txt";
    let b_file = "sample-src/ch2/fizzbuzz_rust.txt";
    // 読み込み
    let a_txt = fs::read_to_string(a_file).unwrap();
    let b_txt = fs::read_to_string(b_file).unwrap();
    let a_txt = a_txt.trim();
    let b_txt = b_txt.trim();

    // 比較
    if a_txt == b_txt {
        println!("OK");
    } else {
        println!("NG");
    }
}
