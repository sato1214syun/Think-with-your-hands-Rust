// メモリ参照カウンター
// ダミーのmain関数
fn main() {}

/*
sec02ではBox<T>を用いた単方向リストを実装した
slist.rsで実装したslist::Listのメソッドのうち
pushメソッドは、loopで末尾のノードを探して、新しいノードを追加しているため計算量が多い
そこで、リストの末尾にも簡単にデータを追加できるようにList構造体のフィールドとして、
先頭を保持するheadの他に、
末尾を保持するfootを追加する

その場合、headとfootが同じノードを指すケースが発生するため、所有権の問題が発生する
これを解決するために、Rc<T>型を用いる
Rc<T>型はBox<T>同様にヒープ領域に値Tのメモリを確保するが、
参照カウント方式でメモリ管理を行うため、1つのオブジェクトに対して複数の所有者を持つことができる
RcはReference Countingの略
https://doc.rust-jp.rs/book-ja/ch15-04-rc.html
*/

// Rc<T>型で参照カウンター方式のメモリ管理を理科するためのテストコード
// rc_test_err.rs エラーのあるプログラム

// fn rc_test_err_main() {
//     // ヒープにi32の値を確保
//     let a_box = Box::new(1000);
//     {
//         let b_box = a_box; // 所有権が移動してしまう
//         println!("{}", b_box);
//     }
//     println!("{}", a_box); // 所有権がb_boxに移動しているのでエラー
// }

// 上記のプログラムをRc<T>型を用いて修正する
// rc_test_fix.rs
#[test]
fn rc_test_fix_main() {
    use std::rc::Rc;
    // ヒープにi32の値を確保
    let a_rc = Rc::new(1000);
    // a_rcの参照カウンタを確認
    println!("a_rc参照数{} (RC<T>作成後)", Rc::strong_count(&a_rc));
    {
        // Rc<T>型はcloneメソッドで所有権を移動せずに参照カウントを増やすことができる
        let b_rc = a_rc.clone(); // RC::clone(&a_rc)とも書ける
                                 // a_rcの参照カウンタを確認、a_rcとb_rcで参照カウンタは共有
        println!("a_rc参照数{} (clone後)", Rc::strong_count(&a_rc));
        println!("b_rc参照数{} (clone後)", Rc::strong_count(&b_rc));
        // b_rcの値を確認
        println!("b_rc={}", b_rc);
    }
    // ここでb_rcがスコープ外で破棄され、a_rcの参照カウンタが-1される
    println!("a_rc参照数{} (b_rc破棄後)", Rc::strong_count(&a_rc));
    println!("{}", a_rc); // RC<T>型なので利用可能
}

// Rc<T>はイミュータブル
// rc_mod_err.rs エラーの出るコード
// fn rc_mod_err_main() {
//     use std::rc::Rc;
//     // ヒープにi32の値を確保
//     let mut a_rc = Rc::new(1000);
//     // a_rcの参照を複製
//     let mut b_rc = a_rc.clone();
//     // ここで値を変更したいが、Rc<T>はイミュータブル
//     *b_rc += 100; // 変更できない
//     println!("{}", b_rc);
// }

// そこで、RefCell<T>型を用いる
/*
RefCell<T>型は、コンパイル時ではなく実行時に借用規則を矯正する方で、書換が可能。
Rc<T>型と組み合わせて、Rc<RefCell<T>>とすることで、Rc<T>型の値Tを書き換えることができる
それでは上記のプログラムをRc<RefCell<T>>を使って修正する
*/
// rc_mod_fix.rs
#[test]
fn rc_mod_fix_main() {
    use std::cell::RefCell;
    use std::rc::Rc;
    // ヒープにi32の値を確保
    let a = Rc::new(RefCell::new(1000));
    // aの参照を複製
    let b = a.clone();
    // a, bの値を変更. RefCell::borrow_mut()は不変な借用を返す
    *b.borrow_mut() += 100;
    // b_rcの値を確認. RefCell::borrow()は不変な借用を返す
    println!("{}", a.borrow());
}

// 循環参照を避けるWeak型
/*
同じRc<T>型の2つのデータで、互いに参照を含んでいた場合、循環参照が発生する
これにより、参照カウントが常に2となり削除されないのでメモリリークが発生する
双方向リストでは前方向のリンク(prev)と次方向のリンク(next)が必要なため、循環参照が問題になる
ここで、Weak<T>型を用いる

Weak<T>型は弱い参照を表し、その時点では所有権を持たない. 対してRc<T>型は強い参照
の参照カウントを増やさず、参照カウントが0になると自動的に解放される
// 双方向リストの定義
struct Node {
    data: isize,
    prev: Option<Weak<RefCell<Node>>>，//弱い参照
    next: Option<Rc<RefCell<Node>>>，// 強い参照
}

上記の構造体を用いて、相互リンクさせるには以下のようにする
Weak<T>型を利用して弱い参照を得るには、Rc::downgradeを用いる
// aのnextにbを指定(強い参照)
a.borrow_mut().next = Some(b.clone());
// bのprevにaを指定(弱い参照)
b.borrow_mut().prev = Some(a.downgrade());

*/

// 弱い参照をたどって値を表示する方法
/*
Rc＜T＞型の参照カウントが減って0になると値が削除されるが、
Weak＜T＞型の弱い参照では、参照先の値が削除されいても気づかない
そこで実際の参照先が参照可能か確かめるために、upgradeメソッドを用いる
upgradeメソッドはOption<Weak<T>>型の値をOption<Rc<T>>型に変換する

// bのprevを得る
match &b.borrow().prev {
    None => {},
    Some(prev) =>｛
        // prevは弱い参照なのでupgradeし値が存在するか確認
        let pa = prev.upgrade().unwrap();
        // aの参照を得た(RC<RefCell<Node>>)ので値を表示
        println!("a.data= {}", pa.borrow().data);
    },
};
*/

// Rc<T>とWeak<T>を組み合わせた双方向リストの実装
// dlist.rs

#[test]
fn dlist_main() {
    // mod dlist;
    // リストに値を追加
    let mut list = dlist::List::new();
    // 末尾に値を追加
    list.push(100);
    list.push(110);
    // 先頭に値を追加
    list.unshift(10);
    list.unshift(20);
    // イテレータで値をすべて表示
    for v in list.iter() {
        println!("{}", v);
    }
}

mod dlist {
    // dlist.rs
    // use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    // 双方向リストの要素一つを表す構造体 --- (*1)
    pub struct Node {
        data: isize,
        // 前後双方向の要素に対し参照を持つ
        // これ以降常に前の要素はRc, 次要素はWeak、またはRcの参照としている
        next: Option<Rc<RefCell<Node>>>,   // 強い参照
        prev: Option<Weak<RefCell<Node>>>, // 弱い参照
    }
    // 双方向リストをまとめる構造体 --- (*2)
    pub struct List {
        // 先頭と末尾の要素を保持することでどちらへの要素追加もO(1)で可能
        head: Option<Rc<RefCell<Node>>>,
        foot: Option<Rc<RefCell<Node>>>,
    }
    // List構造体のメソッドを定義 --- (*3)
    impl List {
        // Listのコンストラクタ
        pub fn new() -> Self {
            Self {
                head: None,
                foot: None,
            }
        }
        // 新しいノードを作成
        fn new_node(v: isize) -> Rc<RefCell<Node>> {
            Rc::new(RefCell::new(Node {
                data: v,
                next: None,
                prev: None,
            }))
        }
        // 末尾に値を追加 --- (*4)
        pub fn push(&mut self, v: isize) {
            let n = List::new_node(v); // 新規ノードを作成
            match self.foot.take() {
                // 末尾がない(リストが空)
                None => {
                    // footとhead両方に新規ノードを設定
                    // footの方にclone()を使う(Rc型が活きる)
                    self.foot = Some(n.clone());
                    self.head = Some(n);
                }
                // 末尾がある(リストが空でない)
                Some(current_foot) => {
                    // 新規ノードをborrow_mut()で可変参照しつつ, prevに既存のfootを設定
                    // prevはweak型なので, downgradeでWeak型に変換
                    n.borrow_mut().prev = Some(Rc::downgrade(&current_foot));
                    // 既存のfootのnextに新規ノードを設定
                    current_foot.borrow_mut().next = Some(n.clone());
                    // footを新規ノードで更新する
                    self.foot = Some(n);
                }
            }
        }
        // 先頭に値を追加 --- (*5)
        pub fn unshift(&mut self, v: isize) {
            let n = List::new_node(v);
            match self.head.take() {
                // 先頭がない(リストが空)
                None => {
                    // footとheadに新規ノードを設定
                    self.foot = Some(n.clone()); // 後ろ側の要素は参照する
                    self.head = Some(n);
                }
                // 先頭がある場合
                Some(current_head) => {
                    // 既存のheadノードをborrow_mut()で可変参照しつつ,
                    // そのprevに新規ノード(downgradeでWeak型に変換)を設定
                    current_head.borrow_mut().prev = Some(Rc::downgrade(&n));
                    // 新規ノードをborrow_mut()で可変参照しつつ, nextに既存のheadノードを設定
                    n.borrow_mut().next = Some(current_head);
                    // headを新規ノードで更新する
                    self.head = Some(n);
                }
            }
        }
        // ListIterオブジェクトを返すメソッド --- (*6)
        pub fn iter(&mut self) -> ListIter {
            match &self.head {
                None => ListIter { current_node: None },
                Some(head) => {
                    // headの参照を外して、ListIterに渡す
                    let head = head.clone();
                    ListIter {
                        current_node: Some(head),
                    }
                }
            }
        }
    }
    // イテレータのための構造体 --- (*7)
    pub struct ListIter {
        pub current_node: Option<Rc<RefCell<Node>>>,
    }
    // イテレータの実装 --- (*8)
    // Iteratorトレイトを構造体に適用し、Item型とnextメソッドを実装する
    impl Iterator for ListIter {
        type Item = isize;
        // nextメソッドで次の要素を返す
        fn next(&mut self) -> Option<Self::Item> {
            match self.current_node.take() {
                None => None,
                Some(current_node) => {
                    // &RefCell<T> = &*Rc<RefCell<T>>と同じような操作(&RefCell<T>のラップがRef<T>?
                    // Refは値と参照ポインタborrowを持つ構造体のようだ)
                    let borrowed_current_node = current_node.borrow();
                    // 次のノードを取得
                    match &borrowed_current_node.next {
                        // 次のノードがない場合はNoneを設定
                        None => self.current_node = None,
                        // 次のノードがある場合はそのノードを設定
                        Some(next) => self.current_node = Some(next.clone()),
                    }
                    Some(borrowed_current_node.data)  // 現在のノードの値を返す
                }
            }
        }
    }
}

/*
ここで出てきたRc<RefCell<T>>ト似たような型にArc<Mutex<T>>がある
Arc<T>はRc<T>のスレッドセーフ版で、Mutex<T>はRefCell<T>のスレッドセーフ版
並列並列処理で使用する
*/
