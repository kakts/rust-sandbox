
/**
 * actix-webによるwebサーバ
 */

// {}の中に書いた名前をコード内で直接使えるようになる
use actix_web::{web, App, HttpResponse, HttpServer};

// フォーム処理用
use serde::Deserialize;

// フォームから入力される値を表す構造体
// #[derive(Deserialize)]を型定義の前に付与すると
// serdeクレーとはコンパイル時に型を解析して、POSTに使用する形式のデータからその型のデータを取り出すコードを自動生成する
// これとは逆にRustの値を構造化された形式で書き出すためのSerializeも用意されている
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn main() {
    // サーバの起動
    /**
     * || {}は　クロージャ式
     * 
     * 関数のように呼び出せる値
     * このクロージャは引数を取らないが、取る場合には引数の名前を||に書く
     * 
     * {...}はクロージャのボディ部になる
     * 
     * ここでは、引数を取らないクロージャでクロージャのボディ部はApp::newしてルーティング設定を指定しているクロージャとなる
     */
    let server = HttpServer::new(|| {
        // ルーティング指定
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to addresss")
        .run().expect("error running server");
}

// GET:/ でリクエストした時のハンドラ
fn get_index() -> HttpResponse {

    // HTMLのレスポンス
    /**
     * ダブルクオートが複数必要なので　「raw string」構文を用いる
     * rの後に0個以上のハッシュマークが続いた後にダブルクオートで始まる
     * この文字列の中ではダブルクオートを含む任意の文字列をエスケープなしで使える
     */
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

/**
 * フォームからのPOST用のハンドラ
 * POST:/gcd
 */
fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = 
        format!("The greatest common divisor of the numbers {} and {} \
            is <b>{}</b>\n", form.n, form.m, gcd(form.n, form.m));
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
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