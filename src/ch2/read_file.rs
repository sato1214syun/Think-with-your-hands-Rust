use std::env;
use std::fs;

fn main() {
    // 引数をベクターとして得る
    let args: Vec<String> = env::args().skip(1).collect();
    // ファイル名の指定があるか確認
    if args.len() < 1 {
        eprintln!("Usage: read.file <file>");
        return;
    }

    // 最初の要素を取る
    let filename = &args[0];
    // ファイルの中身を表示
    let text = fs::read_to_string(filename).unwrap();
    println!("{}", text);
}