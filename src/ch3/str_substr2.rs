use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let pr = "知恵は武器よりもk勝ちがある。";

    //先頭2文字(6バイト)の部分文字列お得る
    let sub3: String = pr.chars().take(2).collect();
    println!("先頭2文字: {}", sub3);

    // 「武器」の部分の部分文字列を取得する
    let pr_chars: Vec<char> = pr.chars().collect();
    println!("4-5文字目: {}", &pr_chars[3..=4].into_iter().collect::<String>());

    // char_indicesを使って「武器」の部分の部分文字列を取得する
    let sub_start = pr.char_indices().nth(3).unwrap().0;
    let sub_end = pr.char_indices().nth(5).unwrap().0;
    println!("4-5文字目: {}", &pr[sub_start..sub_end]);

    // char_indicesとnthを使って「武器」の部分の部分文字列を取得する
    let sub_start = pr.grapheme_indices(true).nth(3).unwrap().0;
    let sub_end = pr.grapheme_indices(true).nth(5).unwrap().0;
    println!("4-5文字目: {}", &pr[sub_start..sub_end]);

    // unicode_segmentationのgrapheme_indicesとnthを使って「武器」の部分の部分文字列を取得する
    // unicode_segmentationはunicodeの書記素クラスタ(grapheme cluster)をうまく処理できる
    // 例えば国旗の絵文字など1文字分がコードポイント2つ分で表現される文字列を正しく処理できる
    let s: &str = "🇯🇵JP😀";
    println!(
        "{:?}",
        s.char_indices().collect::<Vec<(usize, char)>>()
        // [(0, '🇯'), (4, '🇵'), (8, 'J'), (9, 'P'), (10, '😀')]
    );
    println!(
        "{:?}",
        s.grapheme_indices(true).collect::<Vec<(usize, &str)>>(),
        // [(0, "🇯🇵"), (8, "J"), (9, "P"), (10, "😀")]
    );
}
