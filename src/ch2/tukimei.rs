use std::collections::HashMap;

fn main() {
    // 旧暦月名
    let tuki = [
        "睦月",
        "如月",
        "弥生",
        "卯月",
        "皐月",
        "水無月",
        "文月",
        "葉月",
        "長月",
        "神無月",
        "霜月",
        "師走",
    ];

    // HashMapを作成
    let mut tuki_map: HashMap<&str, usize> = HashMap::new();
    // 月名をHashMapに追加
    for (i, v) in tuki.iter().enumerate() {
        tuki_map.insert(v, i + 1);
    }
    // 要素を選んで表示する
    let test_array = ["睦月", "卯月", "霜月", "エラ月"];
    for input in test_array {
        // pythonと同じでキーの存在判定をgetで行える
        // getはOption方を返すので、ifを使うよりmatchを使うのが一般的
        match tuki_map.get(input) {
            None => println!("{}は存在しません", input),
            Some(v) => println!("{} = {}月", input, v),
        }
    }
}
