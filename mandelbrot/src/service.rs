use num::Complex;

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
    let mut z = Complext {re: 0.0, im: 0.0};
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
    let mut z = Complex { re: 0.0, im: 0.0 }
    loop {
        z = z * z + c;
    }
}