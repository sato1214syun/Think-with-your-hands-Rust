use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
// use std::fs;

fn main() {
    // 引数をベクターとして得る
    let args = env::args().skip(1);
    // ファイル名の指定があるか確認
    if args.len() < 1 {
        eprintln!("Usage: read.file <file>");
        return;
    }

    let mut total = 0.0;
    for fname in args {
        // read_to_stringを使う場合
        // let text = fs::read_to_string(fname).unwrap();
        // let lines = text.lines();
        // for line in lines {

        // BufReaderを使う場合
        let line_iter = BufReader::new(File::open(fname).unwrap()).lines();
        for line in line_iter {
            let n = match line.unwrap().parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };
            total += n;
        }
    }
    println!("Total: {}", total);
}
