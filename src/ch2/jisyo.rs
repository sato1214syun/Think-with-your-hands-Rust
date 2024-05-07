use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let dict_file = "sample-src/ch2/ejdict-hand-utf8.txt";
    let args: Vec<String> = env::args().skip(1).collect();

    let word = match args.get(0) {
        Some(v) => v,
        None => {
            eprintln!("Usage: jisyo <word>");
            return;
        }
    };

    // ファイルを開く
    let fp = File::open(dict_file).unwrap();
    // BufReaderで1行ずつ読み込む
    let reader = BufReader::new(fp);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(word) {
            println!("{}", line);
        }
    }
}
