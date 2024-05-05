use regex::Regex;

fn main() {
    let showa_start = 1926;
    let heisei_start = 1989;
    let reiwa_start = 2019;
    for y in 1926..=2026 {
        let wareki: String;
        if y < 1989 {
            wareki = format!("昭和{}年", y - showa_start + 1);
        } else if y < 2019 {
            wareki = format!("平成{}年", y - heisei_start + 1);
        } else {
            wareki = format!("令和{}年", y - reiwa_start + 1);
        }

        // 各和暦が1年の場合は「元年」に変換
        let re = Regex::new(r"^\D*1年$").unwrap();
        match re.find(&wareki) {
            Some(_i) => println!("西暦:{} = {}", y, wareki.replace("1年", "元年")),
            None => println!("西暦:{} = {}", y, wareki),
        }
    }
}
