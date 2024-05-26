// 文字列をバイナリエディタ風に出力する

fn main() {
    hex_dump("自分の口を見張る人は自分の命を守る。");
}

fn hex_dump(s: &str) {
    // 1バイトずつ表示する
    for (i, c) in s.bytes().enumerate() {
        // アドレスを表示
        if i % 16 == 0 {
            print!("{:08x}|", i);
        }
        // 4桁ごとに区切り文字をいれる
        if i % 4 == 3 {
            print!("{:02x}|", c);
        } else {
            print!("{:02x} ", c);
        }
        // 16バイトごとに改行
        if i % 16 == 15 {
            println!();
        }
    }
    println!();
}
