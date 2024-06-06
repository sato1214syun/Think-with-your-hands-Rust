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
    rule if() -> Node = "if" _ v:if_comparison() { v }
    rule if_comparison() -> Node
        = if_elif() / if_else() / if_true_only()
    rule if_elif() -> Node
        = comparison:calc() t:block() lf() "elif" _ f: if_comparison()
        { Node::if_(comparison, t, vec![f]) }
    rule if_else() -> Node
        = comparison:calc() t:block() lf() "else" _ f:block()
        { Node::if_(comparison, t, f) }
    rule if_true_only() -> Node
        = comparison:calc() t:block()
        { Node::if_(comparison, t, vec![]) }
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
    rule comp() -> Node
        = l:expr() "==" _ r:comp() { Node::calc('=', l, r) }
        / l:expr() "!=" _ r:comp() { Node::calc('!', l, r) }
        / l:expr() ">" _ r:comp() { Node::calc('>', l, r) }
        / l:expr() ">=" _ r:comp() { Node::calc('g', l, r) }
        / l:expr() "<" _ r:comp() { Node::calc('<', l, r) }
        / l:expr() "<=" _ r:comp() { Node::calc('l', l, r) }
        / expr()
    rule expr() -> Node
        = l:term() "+" _ r:calc() { Node::calc('+', l, r) }
        / l:term() "-" _ r:calc() { Node::calc('-', l, r) }
        / term()
    rule term() -> Node
        = l:val() "*" _ r:term() { Node::calc('*', l, r) }
        / l:val() "/" _ r:term() { Node::calc('/', l, r) }
        / l:val() "%" _ r:term() { Node::calc('%', l, r) }
        / val()
    rule val() -> Node
        = "(" _ v:calc() _ ")" _ { v }
        / v:number() _ { Node::Number(v) }
        / v:word() _ { Node::GetVar(v) }
    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }
    rule word() -> String // 変数名の定義 --- (*10)
        = v:$(['a'..='z'|'A'..='Z'|'_']+ ['0'..='9']*)
        { String::from(v) }

    rule end_of_line() = [';' | '\n']+ _ // 文の区切り
    rule lf() = _ ['\n']* _ // 改行
    rule _ = [' ' | '\t']* // 空白文字
});
