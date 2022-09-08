use std::str::FromStr;
use num::Complex;

fn main() {
    println!("Hello, world!");

    // TODO
    // match parse_pair("123,456", ',') {
    //     Some(l, r) => println!("parse pair result is {:?}, {:?}", l, r),
    //     None => println("parse pair result is None")
    // }
    // println!("parsed pair result is {:b}", parsed_pair)

    let parsed_complex = parse_complex("1.25,-0.0625");
    println!("parsed complex result is {:?}", parsed_complex);
}

/// sが適切な形であればSome<(x,y)>を返す　そうでなければNone
/// <T: FromStr> は FromStrトレイトを実装する任意の型Tに対して　と読む
/// Option<(T, T)> NoneかSome((v1, v2))の値となる。 (v1, v2)は型Tの値2つのタプル
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {

    /// 文字列の中からseparatorに合致する文字を探す。
    /// findがNoneを返す場合は、セパレータ文字が文字列には現れなかったことを意味し、Noneを返し、パース失敗を表す
    match s.find(separator) {
        None => None,
        Some(index) => {
            // indexはseparator文字の位置を表す
            // separatorの文字の前後を取り出した文字列のスライスをとり、型Tのタプルを作る
            // これに対してマッチングを行う
            // _は何にでもマッチし、その値を無視する
            match (T::from_str(&s[..index]), T::from_str(&s[index +1..])) {
                (Ok(l), Ok(r)) => Some((l, r)), // 双方のパースが成功した場合
                _ => None
            }
        }
    }
}

/// カンマで分けられたfloatのペアをパースして複素数を返す
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None
    }
}

///
///limitを繰り返し回数の上限として、cがマンデルブロ集合に含まれるかを判定する
///
/// cがマンデルブロ集合に含まれないならSome(i)を返す
/// iはcが原点を中心とする半径2の縁から出るまでにかかった繰り返し回数となる
/// cがマンデルブロ集合に含まれているらしい(繰り返し上限に達しても、cがマンデルブロ集合に含まれないことを示せなかった場合)
/// Noneを返す
/// 
/// 戻り値はOption<usize>
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..limit {

        // 半径2の円からでたかどうか
        // zの原点からの距離の2乗
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}


/**
 * xの値に応じて、xは0に近づくか、1のままか、無限大に近づくかのいずれか
 */
fn square_loop(mut x: f64) {
    // loop
    loop {
        x = x * x;
    }
}

fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

/**
 * 複素数対応版ループ
 */
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<i32>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<i32>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex {re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex(", -0.0625"), None);
}