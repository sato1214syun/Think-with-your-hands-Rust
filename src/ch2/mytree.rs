use std::{env, path};

// ディレクトリ構成をツリー形式で表示するプログラム
fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().skip(1).collect();
    let default_dir = String::from("./src");
    let target_dir = args.get(0).unwrap_or(&default_dir);

    // PathBuffに変換
    let target = path::PathBuf::from(target_dir);
    println!("{}", target.display());
    tree(&target, 0);
}

// 再帰的にファイル一覧をツリー形式で表示
fn tree(target: &path::PathBuf, level: isize) {
    // ファイル一覧を取得
    let files = target.read_dir().expect("no such directory");

    // エントリを走査
    for ent in files {
        // PathBufを得る
        let path = ent.unwrap().path();
        // level分インデントする
        for _ in 1..=level {
            print!("|   ");
        }
        // ファイル名を文字列に変換
        let fname = path.file_name().unwrap().to_string_lossy();
        // pathがディレクトリの場合再帰的に表示
        if path.is_dir() {
            println!("|-- <{}>", fname);
            tree(&path, level + 1);
            continue;
        }
        println!("|-- {}", fname)
    }
}
