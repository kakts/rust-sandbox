use std::str::FromStr;
use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn main() {
    println!("Hello, world!");

    let parsed_complex = parse_complex("1.25,-0.0625");
    println!("parsed complex result is {:?}", parsed_complex);
}

/// 大きさがbounds で指定されたバッファpixelsをfilenameで指定されたファイルに書き出す
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    // ファイルオープンし、画像をそのファイルに書き出す
    let output = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            return Err(e);
        }
    };
    let encoder = PNGEncoder::new(output);

    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))

    Ok(())
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

/// 出力される画像のピクセルの位置をとり、対応する複素平面上の点を返す
/// pixelは画像上の特定のピクセルを（行,列)ペアの形で指定する
/// 仮引数upper_left lower_rightは出力画像に描画する複素平面を左上と右下で指定する
fn pixel_to_point(bounds: (usize, usize),
                    pixel: (usize, usize),
                    upper_left: Complex<f64>,
                    lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    // imが引き算となっている理由。 上に動くとpixel.1は増えるが、虚部は小さくなるため
    // pixel.0 pixel.1はタプルの要素を参照
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

fn render(pixels: &mut [u8],
            bounds: (usize, usize),
            upper_left: Complex<f64>
            lower_right: Complex<f64
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for now in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row),
                upper_left, lower_right);
            pixels[row * bounds.0 + column] = 
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
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
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex {re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex(", -0.0625"), None);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175),
                                Complex {re: -1.0, im: 1.0},
                                Complex {re: 1.0, im: -1.0}),
                                Complex {re: -0.5, im: -0.75});
}