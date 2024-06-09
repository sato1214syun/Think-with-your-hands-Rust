// 暗号化ツールを作ってみよう
// ダミーのmain関数
fn main() {}

/*
実用的な暗号化ツールを作る
aesクレートのAES-256という暗号化方式を用いる
他にもクレートが必要
- aes: AESによる暗号化
- block-modes: AES-256は固定長のブロックを暗号化するため、ブロック長よりも長いメッセージの暗号化には暗号利用モードと呼ばれる操作が必要。そのためにこのクレートを使う
- base64: バイナリデータを64文字の英数記号で表現するBASE64の機能を提供
- sha2: SHA-256などのハッシュ関数を計算する。AES-256の暗号化には32バイトのキーと16バイトの初期ベクトルが必要。
        ここでは可変長のパスワードを32バイトに要約するためハッシュ関数を使う。
        本来はPKCS#5に基づいて鍵導出関数PBKDF2(Password-Based Key Derivation Function 2)を使うべきだが、簡略化してSHA-256を使う
- getrandom: 初期ベクトルをランダムに生成するのに使う
*/

// コマンドラインから使える暗号化ツール
// cipher_cmd

mod cipher_str {
    use aes::Aes256;
    use base64::{engine::general_purpose, Engine as _};
    use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
    use sha2::{Digest, Sha256};

    // ブロックの暗号化の種類と暗号利用モードを指定 --- (*1)
    // typeで型のエイリアスを作成
    type AesCbc = Cbc<Aes256, Pkcs7>;
    // SALTは今回固定するのでハードコーディング
    const SALT: &str = "LFsMH#kL!IfY:dcEz9F/dvj17nUN";

    // passwordでdataを暗号化する関数 --- (*2)
    pub fn encrypt(password: &str, data: &str) -> String {
        // パスワードを固定長のキーに変換 --- (*3)
        let key = get_key(password);
        let init_vec = gen_init_vec();
        // 暗号化 --- (*4)
        let cipher = AesCbc::new_from_slices(&key, &init_vec).unwrap();
        let result = cipher.encrypt_vec(data.as_bytes());
        //暗号化した結果の前にivを足す --- (*5)
        let mut ivres: Vec<u8> = vec![];
        ivres.extend(init_vec);
        ivres.extend(result);
        // BASE64でエンコードして戻す --- (*6)
        general_purpose::STANDARD.encode(ivres)
    }

    // 初期化ベクトル(IV)をランダムに生成 --- (*7)
    fn gen_init_vec() -> Vec<u8> {
        let mut res: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        getrandom::getrandom(&mut res).unwrap();
        res
    }

    // パスワードから32バイトの暗号キーを得る --- (*8)
    fn get_key(password: &str) -> Vec<u8> {
        let pw: String = format!("{}::{}", password, SALT);
        let mut h = Sha256::new();
        h.update(pw.as_bytes());
        h.finalize().to_vec()
    }

    // 復号化する関数 --- (*9)
    pub fn decrypt(password: &str, data: &str) -> String {
        // パスワードから暗号キーを得る
        let key = get_key(password);
        let bytes = general_purpose::STANDARD.decode(data).unwrap();
        // データの先頭にある初期化ベクトルを取り出す
        let init_vec = &bytes[..16];
        // 復号化する
        let cipher = AesCbc::new_from_slices(&key, init_vec).unwrap();
        let result = cipher.decrypt_vec(&bytes[16..]).unwrap();
        String::from_utf8(result).unwrap()
    }

    #[cfg(test)]
    mod chipper_tests {
        use super::*; // 外の要素を取り込む

        #[test]
        fn enc_dec_test() {
            // 関数をテストする --- (*10)
            let password = "abcd";
            let data = "穏やかな心は体に良い。";
            let enc = encrypt(password, data);
            println!("暗号化: {}", enc);
            let dec = decrypt(password, &enc);
            println!("復号化: {}", dec);
            assert_eq!(data, dec);
        }
    }
}

// 暗号化ツールのメインファイル
// cipher_cmd/main.rs
#[test]
fn cipher_cmd_main() {
    // コマンドラインから使う --- (*1)
    // let args: Vec<String> = std::env::args().skip(1).collect();
    // let args: Vec<String> = vec![
    //     "enc".to_string(),
    //     "abcd".to_string(),
    //     "こんにちは".to_string(),
    // ];
    let args: Vec<String> = vec![
        "dec".to_string(),
        "abcd".to_string(),
        "CX1UmICTfkvYixdnBBudfcBM4H9NVSzmWnolz37SrJg=".to_string(),
        // "cHP5lunuRg/8EUidPcygOkTscZ6VJr1AjgDr/yICLHU=".to_string(),
    ];
    if args.len() < 3 {
        show_usage();
        return;
    }
    // コマンドライン引数から値を得る
    let method = String::from(args[0].trim());
    let password = String::from(args[1].trim());
    let data = String::from(args[2].trim());
    // 暗号化・復号化 --- (*2)
    let result = match &method[..] {
        "enc" => cipher_str::encrypt(&password, &data),
        "dec" => cipher_str::decrypt(&password, &data),
        _ => {
            show_usage();
            return;
        }
    };
    println!("{}", result);
}

fn show_usage() {
    println!("Usage: cipher_cmd enc|dec password data");
}

// WebAssemblyにしてブラウザで動かしてみる
// cipher_browserディレクトリを参照