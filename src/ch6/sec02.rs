// 単方向リストでメモリ管理を理解する
// ダミーのmain関数
fn main() {}

// 単方向リストの実装(コンパイルエラー)
// list_err.rs

// error[E0072]: recursive type `Node` has infinite size
// linkにNodeを持つ構造体を定義すると、再帰的な型になるためコンパイルエラーになる
// 再帰的な型を使用すると、スタック領域のメモリを無限に使用するため 、メモリサイズが決まらない
// pub struct NodeErr {
//     data: i64,
//     link: Option(Node),
// }

// #[test]
// fn list_err_main() {
//     let mut c = Node {
//         data: 30,
//         link: NodeErr,
//     };
//     println!("{}", c.data);
// }

// ヒープ領域にメモリを確保するBox<T>を使う
// list_fix.rs

pub struct Node {
    data: i64,
    link: Option<Box<Node>>,
}
// 手軽に単方向リストを生成する関数
fn node(v: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node {
        data: v,
        link: link,
    }))
}
#[test]
fn list_fix_main() {
    // 単方向リストを生成
    let c = node(10, node(20, node(30, None))).unwrap();

    // 先頭から各要素をたどって、値を表示
    let mut p = &c;
    loop {
        println!("{}", p.data);
        // pに次の要素の参照を代入
        match p.link {
            None => break,
            Some(ref link) => p = &link,
        }
    }
}

// 構造体の代わりに列挙型enumを使用するバージョン
// list_enum.rs

// 列挙型でNodeを定義
enum NodeEnum {
    Empty,
    Cons(i64, Box<NodeEnum>),
}

// 列挙型を手軽に使えるようuseでConsとEmptyを宣言
use NodeEnum::{Cons, Empty};
fn node_enum(v: i64, link: Box<NodeEnum>) -> Box<NodeEnum> {
    Box::new(Cons(v, link))
}
#[test]
fn list_enum_main() {
    // 単方向リストを生成
    let c = node_enum(10, node_enum(20, node_enum(30, Box::new(Empty))));

    // 先頭から各要素をたどって、値を表示
    let mut ptr: &Box<NodeEnum> = &c;
    loop {
        // &Box<NodeEnum>からNodeEnumを取り出す
        // Box<T>はその値への参照(ボインタ)である
        // Box<T>からTを取り出すには、参照外し<デリファレンス>を行う(*を使う)
        // では&**ptrは何をしているのか
        // ptrは"&Box<NodeEnum>"
        // *ptrは"Box<NodeEnum>"
        // **ptrは"NodeEnum" (Box<T>のTを取り出すための参照外し)
        // &**ptrは"&NodeEnum"
        // つまり、ptrの指す先のNodeEnumを参照として取り出している
        let current_node: &NodeEnum = &**ptr;
        match current_node {
            Empty => break,
            Cons(v, ref link) => {
                println!("{}", v);
                ptr = &link;
            }
        }
    }
}

// 参照外しの例
// box_test.rs
#[test]
fn box_test_main() {
    // ヒープ領域に100を確保してポインタを返す
    let x_box = Box::new(100);
    // 参照外しをすれば値を取り出せる
    let x_val = *x_box;
    println!("{}", x_val); // 100
}

// 単方向リストにメソッドを実装する
// slist
// モジュールはslist.rsに記載
// mod slist;

#[test]
fn slist_main() {
    // リストを作成
    let mut list = slist::List::new();
    // 値を末尾に追加
    list.push(100);
    list.push(200);
    // 先頭に値を追加
    list.unshift(10);
    list.unshift(20);
    // 任意のインデックスを取得
    println!("{}", list.get(0).unwrap());
    println!("{}", list.get(1).unwrap());
    println!("{}", list.get(2).unwrap());
    println!("{}", list.get(3).unwrap());
}

mod slist {
    // 単方向リストの要素1つを表す構造体
    pub struct Node {
        data: isize,
        link: Option<Box<Node>>,
    }
    // 単方向リストをまとめる構造体
    pub struct List {
        head: Option<Box<Node>>,
    }
    // List構造体のメソッドを定義
    impl List {
        pub fn new() -> Self {
            // 自身を生成するコンストラクター
            List { head: None }
        }
        // リストの先頭に要素を追加
        pub fn unshift(&mut self, v: isize) {
            let new_node = Node {
                data: v,
                // self.headの所有権をlinkに移したい。
                // "&"mut self(selfの参照)なので、self.headのみでは所有権は取得できないが、self.head.take()で所有権を取得している
                // Option::take()はSomeの値を取り出し、Noneに置き換えるメソッド。
                // つまりself.head.take()はself.headから所有権がある状態で値を取り出し、self.headの中身をNoneに置き換えている。
                // https://mox692.hatenablog.com/entry/2022/05/22/115139
                // https://qiita.com/knknkn1162/items/1d190880efffe3578d92
                link: self.head.take(),
            };
            self.head = Some(Box::new(new_node));
        }
        // リストの末尾に要素を追加
        pub fn push(&mut self, v: isize) {
            // 新規の値
            let new_node = Node {
                data: v,
                link: None,
            };
            match self.head {
                // リストが空の場合
                None => self.head = Some(Box::new(new_node)),
                // リストが空でない場合
                // refは値の参照を取得する
                Some(ref mut head) => {
                    // 末尾のノードを探して新規ノードをリンクする
                    let mut p = head;
                    loop {
                        match p.link {
                            None => {
                                p.link = Some(Box::new(new_node));
                                break;
                            }
                            Some(ref mut next) => p = next,
                        }
                    }
                }
            }
        }
        // 任意のインデックスの値を返す
        pub fn get(&self, index: isize) -> Option<isize> {
            match self.head {
                None => return None, // リストがからの場合
                Some(ref top) => {
                    // 任意のインデックスの値を探す
                    let mut p = top;
                    let mut i = 0;
                    loop {
                        if i == index {
                            // 見つけた
                            return Some(p.data);
                        }
                        match p.link {
                            // 次の要素に
                            None => return None,
                            Some(ref link) => p = link,
                        }
                        i += 1;
                    }
                }
            }
        }
    }
}
