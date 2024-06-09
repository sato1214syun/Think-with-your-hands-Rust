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
