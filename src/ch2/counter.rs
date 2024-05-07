// Rustで人気投票の集計
use std::collections::HashMap;

// 投票データを定数として宣言
const V_DATA: &str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C";
fn main() {

    // 集計用のHashMapを作成
    let mut c_map = HashMap::new();
    // HashMapに初期値を設定
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);
    // 投票データを集計
    for w in V_DATA.split(",") {
        c_map.insert(w, c_map[w] + 1);
    }
    // 集計結果を表示
    for k in ["A", "B", "C"] {
        println!("{}: {:>2}", k, c_map[k]);  // {:>2}で右寄せ表示
    }
}

