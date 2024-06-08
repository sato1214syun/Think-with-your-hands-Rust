use crate::node::Node;
use peg;
// トマト言語の文法定義 --- (*1)
peg::parser!( pub grammar tomato() for str {
    // ルートとなる規則を定義 --- (*2)
    pub rule parse() -> Vec<Node>
        = v:sentences()
    // プログラムで複文が書けるようにする --- (*3)
    // sentences()の動作を定義
    rule sentences() -> Vec<Node>
        // "文法 ** 区切り"とすることで、特定の要素を繰り返し処理できる。
        = sentence() ** end_of_line()
    // 文を定義 --- (*4)
    // sentence()の動作を定義
    rule sentence() -> Node
        // 下記の要素を1文とする(/はOR条件)
        = print() / if() / for() / let() / _ { Node::Nop }

    // print文の定義 --- (*5)
    rule print() -> Node
        // 'print "abc"'の形式で文字列を出力する
        // [^ '"']*は、^が否定なので、"以外の文字の繰り返しを表す
        // _は空白文字を表す
        = "print" _ "\"" v:$([^ '"']*) "\""
        { Node::PrintStr(v.to_string()) }
        // 'print 数式'の形式で数値を出力する
        / "print" _ v:calc()
        { Node::Print(Box::new(v)) }

    // if文の定義 --- (*6)
    // if␣比較式
    rule if() -> Node = "if" _ v:if_comparison() { v }
    // 比較式の中身を定義(elif, else, true_onlyの三種類)
    rule if_comparison() -> Node
        = if_elif() / if_else() / if_only()
    // elifを定義.以下の4つの要素で構成される
    // 1) calc()で演算子を処理
    // 2) t:block()で{␣処理文␣}を定義
    // 3) if()で再帰的にif_comparison()を呼び出す
    // 4) elif␣比較式if_comparison()を定義
    rule if_elif() -> Node
        = comparison:calc() t:block() lf() "elif" _ f: if_comparison()
        { Node::if_(comparison, t, vec![f]) }
    // elseを定義.elifのときの3)と4)を変更したもの。else以降は他の条件が続かないため、4)がblock()となっている
    rule if_else() -> Node
        = comparison:calc() t:block() lf() "else" _ f:block()
        { Node::if_(comparison, t, f) }
    // if文onlyの構文を定義(if is_trueなど)
    rule if_only() -> Node
        = comparison:calc() t:block()
        { Node::if_(comparison, t, vec![]) }
    // {}ブロックの定義
    rule block() -> Vec<Node>
        = "{" _ v:sentences() _ "}" _ { v }

    // for文の定義 --- (*7)
    rule for() -> Node
        = "for" _ w:word() _ "=" _ start:number() _
          "to" _ end:number() _ body:block()
        { Node::For(w, start, end, Box::new(body)) }

    // 代入文の定義 --- (*8)
    rule let() -> Node
        = w:word() _ "=" _ v:calc()
        { Node::SetVar(w, Box::new(v))}

    // 計算処理 --- (*9)
    rule calc() -> Node = comp()
    // 比較演算子の処理
    rule comp() -> Node
        = l:expr() "==" _ r:comp() { Node::calc('=', l, r) }
        / l:expr() "!=" _ r:comp() { Node::calc('!', l, r) }
        / l:expr() ">" _ r:comp() { Node::calc('>', l, r) }
        / l:expr() ">=" _ r:comp() { Node::calc('g', l, r) }
        / l:expr() "<" _ r:comp() { Node::calc('<', l, r) }
        / l:expr() "<=" _ r:comp() { Node::calc('l', l, r) }
        / expr()
    // +, -
    rule expr() -> Node
        = l:term() "+" _ r:calc() { Node::calc('+', l, r) }
        / l:term() "-" _ r:calc() { Node::calc('-', l, r) }
        / term()
    // *. /, %
    rule term() -> Node
        = l:val() "*" _ r:term() { Node::calc('*', l, r) }
        / l:val() "/" _ r:term() { Node::calc('/', l, r) }
        / l:val() "%" _ r:term() { Node::calc('%', l, r) }
        / val()
    // 括弧と数値, 変数のの処理
    rule val() -> Node
        = "(" _ v:calc() _ ")" _ { v }
        / v:number() _ { Node::Number(v) }
        / v:word() _ { Node::GetVar(v) }
    // 数値の定義
    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }
    // 変数名の定義 --- (*10)
    rule word() -> String
        = v:$(['a'..='z'|'A'..='Z'|'_']+ ['0'..='9']*)
        { String::from(v) }

    rule end_of_line() = [';' | '\r'| '\n']+ _ // 文の区切り
    rule lf() = _ ['\r' | '\n']* _ // 改行
    rule _ = [' ' | '\t']* // 空白文字
});
