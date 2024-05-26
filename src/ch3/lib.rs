// テストコードの書き方
// ダミーのmain関数を定義
fn main() {}

// テストコマンド: cargo test を実行したときにビルド対象であることを明示する
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn calc_test1() {
        assert_eq!(100 * 2, 200);
        assert_eq!((1 + 2) * 3, 9);
        assert_eq!(1 + 2 * 3, 7);
    }

    #[test]
    fn calc_test2() {
        assert_eq!(2 * 3, 6);
        // わざと失敗
        assert_eq!(2 * 3, 7);
    }

    #[test]
    fn array_test() {
        // 数値配列を初期化
        let a1 = [100, 200, 300];
        let a2 = [100, 200, 300];
        assert_eq!(a1, a2);
        // Stringの配列を初期化
        let a3 = [
            "リンゴ".to_string(),
            "バナナ".to_string(),
        ];
        let a4 = [
            String::from("リンゴ"),
            String::from("バナナ"),
        ];
        assert_eq!(a3, a4);
    }

    #[test]
    fn vec_test() {
        // Vec<str> を初期化
        let v1 = vec!["apple", "banana", "mango"];
        let mut v2: Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banana");
        v2.push("mango");
        assert_eq!(v1, v2);
    }
}

// 構造体のテスト
// Debugを指定するとフォーマッタを利用して値を出力できるようになる
// PartialEqを指定すると構造体の各要素を比較できるようになる
#[derive(Debug, PartialEq)]
struct GItem {
    name: String,
    price: i64
}

#[cfg(test)]
mod tests2 {
    use super::*; // スコープの外側の要素(今回はGItem)を利用するために必要
    #[test]
    fn item_test() {
        // 構造体を初期化
        let apple1 = GItem {
            name: String::from("リンゴ"),
            price: 2400,
        };
        let mut apple2 = GItem {
            name: "リンゴ".to_string(),
            price: 0,
        };
        apple2.price = 2400;

        // 構造体のフィールドを比較
        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);

        // 構造体全体を直接比較
        assert_eq!(apple1, apple2);
    }
}
