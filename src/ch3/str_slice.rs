// 文字列をバイナリエディタ風に出力する

fn main() {
    let pr = "知恵は武器よりもk勝ちがある。";
    //先頭2文字(6バイト)の部分文字列お得る
    println!("先頭2文字: {}", &pr[0..6]);
    // 「武器」の部分の部分文字列を取得する
    println!("4-5文字目: {}", &pr[9..15]);
}