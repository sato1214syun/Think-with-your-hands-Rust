#include<stdio.h>

// Rustで定義した関数を定義
int rust_mul(int a, int b);

// Rustで定義した関数を実行
int main() {
    printf("%d\n", rust_mul(10, 8));
    printf("%d\n", rust_mul(9, 9));
    return 0;
}

/*
コンパイルして実行するコマンド(macOS/Linux)
rustc --crate-type="dylib" src/ch6/mycalc_rs_test/mycalc.rs -o libmycalc.so
gcc -o mycalc_test src/ch6/mycalc_rs_test/mycalc_test.c ./libmycalc.so
// 実行
./mycalc_test

コンパイルして実行するコマンド(Windows)
Visual Studio 2017以降をインストールして、x64 Native Tools Command Promptを起動し、以下を実行
rustc --crate-type="dylib" src/ch6/mycalc_rs_test/mycalc.rs
cl src/ch6/mycalc_rs_test/mycalc_test.c mycalc.dll.lib /source-charset:utf-8
// 実行
./mycalc_test.exe
*/