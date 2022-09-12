fn main() {
    println!("Hello, world!");
    check_ope();
    sample_array();
    sample_slice_method();
}


/// チェック付き演算
fn check_ope() {
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
}

/// 3.6.1 配列
fn sample_array() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    // 全ての値がtrueの長さ10000のbool配列
    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += 1;   
            }
        }
    }
    assert!(!sieve[211]);
    assert!(!sieve[9876]);
}

/// 3.6.1
// メソッド探索時に配列への参照を暗黙にスライスに変換するので、スライスのメソッドを配列に対して直接使える
fn sample_slice_method() {
    let mut chaos = [3, 5, 4, 1, 2];
    // sortメソッドは実際にはスライスに定義されているが、sortは操作対象を参照として受け取るので、chaosに対して直接実行できる
    // 呼び出し時に暗黙に配列全体を指す&mut[i32]スライスが作られ、それがsortメソッドに渡される
    // len()もスライスのメソッド
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}