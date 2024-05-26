// trait
// ダミーのmain
fn main() {}

// trait: 特性、気質、習性を意味する英単語
// Rustのtraitは、異なる方に対して共通の振る舞い(メソッド)を定義するために使う。他の言語のinterfaceに似ている
// 異なる構造体の振る舞いを共通化する事ができる

// trait_TreasureBox.rs
// 宝箱の振る舞いを定義するトレイト
trait TreasureBox {
    fn check_key_no(&self, key_no: i32) -> bool;
    fn result_when_open(&self);
}

// 宝石箱を表現する構造体を定義
struct JewelryBox {
    price: i32,  // 金貨何枚入っているか
    key_no: i32, // 何番の鍵があれば開くか
}
impl TreasureBox for JewelryBox {
    fn check_key_no(&self, key_no: i32) -> bool {
        // 鍵の番号が一致しているかの判定結果を返す
        self.key_no == key_no
    }
    fn result_when_open(&self) {
        println!("宝石箱だった!金貨{}枚入手。", self.price);
    }
}

// 罠を表現する構造体を定義
struct TrapBox {
    damage: i32, // 罠のダメージ
}
impl TreasureBox for TrapBox {
    // 使わない変数key_noがあると警告されるが、先頭に_をつけて対処する
    fn check_key_no(&self, _key_no: i32) -> bool {
        // どんな鍵でも開くのでtrueを返す
        return true;
    }
    fn result_when_open(&self) {
        println!("罠だった。{}のダメージ。", self.damage);
    }
}

// 冒険者が箱を開ける動作
fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.check_key_no(key_no) {
        println!("鍵が合わず宝箱が開きません。");
        return;
    }
    tbox.result_when_open();
}

#[test]
fn trait_treasure_box_main() {
    // いろいろな宝石を準備
    let box1 = JewelryBox {
        price: 30,
        key_no: 1,
    };
    let box2 = TrapBox { damage: 3 };
    let box3 = JewelryBox {
        price: 20,
        key_no: 2,
    };
    // 冒険者が宝箱を手持ちの鍵で開ける
    let my_key = 2;
    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}

// trait_TreasureBox_def.rs
// トレイトによるデフォルトメソッドの定義。宝石箱と空箱を用意して、check_key_no(鍵番号のチェック)をデフォルトメソッドにしてみる。
// 宝箱の振る舞いを定義するトレイト
trait TreasureBox2 {
    // デフォルトメソッドを定義
    fn check_key_no(&self, key_no: i32) -> bool {
        // 冒険者のキー番号と宝箱のキー番号が一致しているかの判定結果を返す
        self.get_key_no() == key_no
    }
    fn result_when_open(&self);
    fn get_key_no(&self) -> i32;
}

// 宝石箱を表現する構造体を定義
struct JewelryBox2 {
    price: i32,  // 金貨何枚入っているか
    key_no: i32, // 何番の鍵があれば開くか
}
impl TreasureBox2 for JewelryBox2 {
    fn result_when_open(&self) {
        println!("宝石箱だった!金貨{}枚入手。", self.price);
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

// 空箱を表現する構造体を定義
struct EmptyBox {
    key_no: i32, // 南蛮の鍵があれば開くか
}
impl TreasureBox2 for EmptyBox {
    fn result_when_open(&self) {
        println!("空箱だった。");
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

// 冒険者が箱を開ける動作
fn open_box2(tbox: &impl TreasureBox2, key_no: i32) {
    if !tbox.check_key_no(key_no) {
        println!("鍵が合わず宝箱が開きません。");
        return;
    }
    tbox.result_when_open();
}

#[test]
fn trait_treasure_box_def() {
    // いろいろな宝石を準備
    let box1 = JewelryBox2 {
        price: 30,
        key_no: 1,
    };
    let box2 = EmptyBox { key_no: 1 };
    let box3 = JewelryBox2 {
        price: 50,
        key_no: 2,
    };
    // 冒険者が宝箱を手持ちの鍵で開ける
    let my_key = 1;
    open_box2(&box1, my_key);
    open_box2(&box2, my_key);
    open_box2(&box3, my_key);
}
