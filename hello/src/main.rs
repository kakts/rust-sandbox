use std::str::FromStr;
use std::env;

fn main() {

    // 可変なローカル変数numbers 空のベクタで初期化
    // Vecはサイズ可変のベクタ型 jsだと配列に相当する
    // 数をベクタの最後に追加できるようにするには変数にmutをつける必要が亜rう
    // この後の処理で Vec<u64>となるが、ここでは書かない 
    let mut numbers = Vec::new();

    /**
     *コマンドライン引数の処理 args()はイテレータを返す。
     *
     */
    for arg in env::args().skip(1) {
        // コマンドラインで受け取った文字列をu64に変換
        // 型u64はFromStrを実装しているので、u64::from_strを読んでコマンドライン引数を解釈できる
        // ここでu64をプッシュしているのでRustはこのベクタの型を推論できる
        // from_strはResultを返す
        // Resultは Ok(v)かErr(e)を返す
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    // 引数の指定がない場合
    if numbers.len() == 0 {
        // eprintln! stderrに出力
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    /**
     * &と＊は相互補完する関係
     * ベクタの所有権がnumbersに残っていて、ループではその要素を借りていることをしめしている
     * &はベクタの2番目以降の要素への参照を借用している
     * *m の *は mを参照解決する演算子
     */
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

/**
 * 
 * 最大公約数を計算する
 * u64 符号なし64ビット整数
 * 
 * mutキーワードをつけると再代入可能
 */
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // ! は関数でなくマクロ呼び出しとなる
    // 真でなければプログラム終了(panic)
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            // ローカル変数 変数の使われ方から型を推論できるので型を書く必要なし
            // 型推論は関数の中でしか行われない
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }

    // 関数の最後がセミコロンなしの式で終わる場合、その式が関数の返り値となる
    n
}

/**
 * gcdのユニットテスト
 * #[test]はこの関数がテスト関数であることをしめす
 * 
 * 通常コンパイルではスキップされるが、cargo testを実行すると、自動的にコンパイルされ、実行される
 */
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 9), 3 * 11)
}