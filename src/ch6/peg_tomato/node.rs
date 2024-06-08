// 文法要素をNode型として定義 --- (*1)
#[derive(Debug, Clone)]
pub enum Node {
    Nop,                                           // 何もしない
    Number(i64),                                   // 数値を表す
    Calc(char, Box<Node>, Box<Node>),              // 計算式
    If(Box<Node>, Box<Vec<Node>>, Box<Vec<Node>>), // if文
    For(String, i64, i64, Box<Vec<Node>>),         // for文
    Print(Box<Node>),                              // print文(計算出力)
    PrintStr(String),                              // print文(定数出力)
    SetVar(String, Box<Node>),                     // 変数代入
    GetVar(String),                                // 変数参照
}
impl Node {
    // 手軽にNode::Calc型を返すヘルパー関数 --- (*2)
    // Nodeの中にNodeがある入れ子構造なので、Box<T>型が必要
    // ヘルパー関数: よく使う処理をまとめておく関数
    // 普通にCalcやIfの中身を書くと長いので、calc/if_関数で短くまとめておく
    pub fn calc(arithmetic_operator: char, l: Node, r: Node) -> Node {
        Node::Calc(arithmetic_operator, Box::new(l), Box::new(r))
    }
    // 手軽にNode::If型を返すヘルパー関数 --- (*3)
    // runner.rsで定義するIf()関数の内部で、run_nodes関数を呼び出している。
    // run_nodes関数はベクタ型をIteratorにして繰り返し処理を子ているため、引数としてBox<Vec<Node>>型が必要
    pub fn if_(comparison_operator: Node, t: Vec<Node>, f: Vec<Node>) -> Node {
        Node::If(Box::new(comparison_operator), Box::new(t), Box::new(f))
    }
}
