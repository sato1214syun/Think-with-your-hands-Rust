fn main() {}

#[test]
fn str_find() {
    let s = "隣の客はよく柿喰う客だ";

    // "柿", "バナナ"を検索
    let target = vec!["柿", "バナナ"];
    for t in &target {
        match s.find(t) {
            Some(i) => println!("{}: {}B", t, i),
            None => println!("{}はなし", t),
        }
    }
}

#[test]
fn str_find_upper() {
    let s = format!(
        "{}{}",
        "There is more happiness in giving ", " than there is in receiving."
    );
    // クロージャーで検索
    let res = s.find(|c:char| c.to_ascii_uppercase() == 'S');
    match res {
        Some(i) => println!("S: {}B", i),
        None => println!("None"),
    }
}


#[test]
fn str_replace() {
    let s = "苦しむ人にはどの日も悪い日である。";
    // 文字列の置換
    let s2 = s.replace("苦しむ", "陽気な");
    let s3 = s2.replace("悪い日", "宴会");
    // 置換前と置換後を表示
    println!("置換前: {}\n置換後: {}", s, s3);
}

#[test]
fn str_replace2() {
    // シャドーイングを使って簡潔に置換する
    let s = "苦しむ人にはどの日も悪い日である。";
    // 文字列の置換
    let s = s.replace("苦しむ", "陽気な");
    let s = s.replace("悪い日", "宴会");
    // 置換前と置換後を表示
    println!("{}", s);
}