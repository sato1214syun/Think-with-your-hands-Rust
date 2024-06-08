use crate::node::Node;
use crate::parser::tomato;
use std::collections::HashMap;

// プログラム全体で使うコンテキストを定義 --- (*1)
struct Context {
    // 変数とその値をHashで保持する
    vars: HashMap<String, i64>,
    output: String, // WASMでは出力結果をJavascriptに渡すためにString型で保持
}

// 構文木を一つ実行する --- (*2)
// Contextを与えることで、変数の操作を可能にしている
// 戻り値はかならずi64
fn run_node(context: &mut Context, node: Node) -> i64 {
    // matchでどのタイプのノードかを判定
    match node {
        // 数値を返す
        Node::Number(v) => v,
        // 計算式を計算する --- (*3)
        Node::Calc(op, l, r) => {
            calc_op(op, run_node(context, *l), run_node(context, *r))
        }
        // 変数の値を得る --- (*4)
        Node::GetVar(name) => {
            match context.vars.get(&name) {
                Some(v) => *v,
                None => 0,
            }
        }
        // 変数の代入
        Node::SetVar(name, node) => {
            let val = run_node(context, *node);
            context.vars.insert(name, val);
            val
        }
        // if文 --- (*5)
        Node::If(cond, true_n, false_n) => {
            let cond_v = run_node(context, *cond);
            if cond_v > 0 {
                run_nodes(context, &*true_n)
            } else {
                run_nodes(context, &*false_n)
            }
        }
        // for文 --- (*6)
        Node::For(name, start, end, body) => {
            let mut r = 0;
            let nodes = *body;
            for i in start..=end {
                context.vars.insert(name.clone(), i);
                r = run_nodes(context, &nodes);
            }
            r
        }
        // 文字列のprint--- (*7)
        Node::PrintStr(v) => {
            context.output += &format!("{}\n", v); // WASM用に習性
            0
        }
        // 数字のprint文 --- (*8)
        Node::Print(node) => {
            let v = run_node(context, *node);
            context.output += &format!("{}\n", v);  // WASM用に習性
            v
        }
        _ => 0,
    }
}
// 演算子に基づいて計算を行う --- (*9)
fn calc_op(op: char, val_l: i64, val_r: i64) -> i64 {
    match op {
        '+' => val_l + val_r,
        '-' => val_l - val_r,
        '*' => val_l * val_r,
        '/' => val_l / val_r,
        '%' => val_l % val_r,
        '=' => {
            if val_l == val_r {
                1
            } else {
                0
            }
        }
        '!' => {
            if val_l != val_r {
                1
            } else {
                0
            }
        }
        '>' => {
            if val_l > val_r {
                1
            } else {
                0
            }
        }
        'g' => {
            if val_l >= val_r {
                1
            } else {
                0
            }
        }
        '<' => {
            if val_l < val_r {
                1
            } else {
                0
            }
        }
        'l' => {
            if val_l <= val_r {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}
// 繰り返しNodeを実行 --- (*10)
fn run_nodes(ctx: &mut Context, nodes: &Vec<Node>) -> i64 {
    let mut result = 0;
    nodes
        .iter()
        .for_each(|node| result = run_node(ctx, node.clone()));
    result
}
// 手軽にプログラムを実行する関数 --- (*11)
pub fn run(src: &str) -> String {
    let nodes;
    match tomato::parse(src) {
        Ok(v) => nodes = v,
        Err(err) => panic!("panic: \n{}", err),
    };
    let mut context = Context {
        vars: HashMap::new(),
        output: String::new(),
    };
    // 実行
    let r = run_nodes(&mut context, &nodes);
    // 結果を返す
    // printで結果を出力していない?
    if context.output == "" {
        return format!("{}", r);
    } else {
        return context.output.clone();
    }
}
