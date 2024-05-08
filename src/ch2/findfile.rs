use std::{env, path};


fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().skip(1).collect();
    let target_dir = args.get(0).expect("[Usage]: findfile <dir> <file>");
    let keyword = args.get(1).expect("[Usage]: findfile <dir> <file>");

    // PathBuffに変換
    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}

fn findfile(target: &path::PathBuf, keyword: &str) {
    // ディレクトリ内のエントリを取得
    let files = target.read_dir().expect("no such directory");

    // エントリを走査
    for dir_entry in files {
        // PathBufを得る
        let path = dir_entry.unwrap().path();
        // pathがディレクトリの場合再帰的に検索
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }
        // ファイル名を文字列に変換
        let file_name = path.file_name().unwrap().to_string_lossy();
        // keywordを含むかチェック
        if !file_name.contains(keyword) {continue;} // pythonのnotはrustでは"!"
        // keywordを含むファイルパスを出力
        println!("{}", path.display());  // サンプルコードでは.to_string_lossy()を使用している。違いは?
    }
}
