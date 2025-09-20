# Rustコードリーディング解説 (4): `solver.rs`, `calculation.rs`, 全体のまとめ

これは、Geminiによる `single-field-inflationary-paramters` プロジェクトのソースコード解説の最終回です。

---

### `src/solver.rs`

このファイルには、関数を引数として受け取る、より高度な関数（高階関数）が定義されています。

```rust
// ... (find_phi_end関数) ...

pub fn find_phi_exit(
    potential: &(impl Potential + ?Sized), 
    phi_end: f64, 
    n_target: f64, // e-foldの目標値
    search_range: (f64, f64),
    precision: f64
) -> Result<f64, &'static str> {

    //　クロージャが使える
    let integrand = |phi: f64| potential.v(phi) / potential.p(phi);
    let efold_num = |phi: f64| simpson(|p| integrand(p), phi_end, phi, precision);
    let find_root = |phi: f64| efold_num(phi) - n_target;
    
    // ... (二分法の実装) ...
}
```

*   **クロージャ (`|...| ...`)**:
    *   `let integrand = |phi: f64| potential.v(phi) / potential.p(phi);` の部分は**クロージャ**と呼ばれる、名前のない関数のようなものです。
    *   `|phi: f64|` が引数の定義、その後の式が本体です。
    *   クロージャの強力な点は、それが定義された場所の環境にある変数（ここでは `potential`）を**キャプチャ**して、内部で利用できることです。
    *   ここでは、e-foldingの計算に必要な被積分関数 `V/V'` を `integrand` という名前のクロージャとして定義しています。これにより、コードが整理され、何を計算しているかが明確になります。

```rust
pub fn simpson<F>(f: F, a: f64, b: f64, precision: f64) -> f64 
    where F: Fn(f64) -> f64 {
    // ...
}
```

*   **ジェネリクスと `where` 句**:
    *   `simpson` 関数は、どんな「`f64` を引数に取り、`f64` を返す関数」でも積分できるように、**ジェネリック**に書かれています。
    *   `<F>` は「何らかの型 `F` を使う」という宣言です。
    *   `where F: Fn(f64) -> f64` は、その型 `F` が満たすべき制約（**トレイト境界**）を指定しています。
        *   `Fn(f64) -> f64` は、`f64` を引数に取り `f64` を返す**関数やクロージャ**が実装するトレイトです。
    *   これにより、`simpson` 関数は特定の関数に縛られることなく、様々な関数（このプロジェクトでは `efold_num` 内で使われるクロージャ）の積分に再利用できます。

---

### `src/calculation.rs`

このファイルは、これまで見てきた各モジュールの部品を組み立てて、一連の計算フローを実行します。

```rust
use crate::potential::Potential;
use crate::solver::{find_phi_end, find_phi_exit};
use crate::cosmology::{epsilon, eta, spectral_index, tensor_to_scalar_ratio};

pub fn calculate_ns_r(
    potential: &dyn Potential,
    n_target: f64,
    precision: f64,
) -> Result<(f64, f64), String> {
    // 1. find_phi_end を呼び出す
    let phi_end = find_phi_end(potential, phi_end_search_range, precision)
        .map_err(|e| format!("Could not find phi_end: {}", e))?;

    // 2. find_phi_exit を呼び出す
    let phi_exit = find_phi_exit(potential, phi_end, n_target, phi_exit_search_range, precision)
        .map_err(|e| format!("Could not find phi_exit: {}", e))?;

    // ... (ns, r の計算) ...
    
    Ok((ns, r))
}
```

*   **`potential: &dyn Potential`**:
    *   `main.rs` から渡されてきた `Box<dyn Potential>` を、参照の形で受け取っています。`&` をつけると `Box` の中身（`dyn Potential`）への参照 `&dyn Potential` となります。
    *   この関数は、受け取ったポテンシャルが `ChaoticPotential` なのか何なのかを知る必要はなく、ただ `Potential` トレイトのメソッドを呼び出すだけです。これがトレイトオブジェクト（動的ディスパッチ）の力です。
*   **`.map_err(|e| ...)`**:
    *   これは `Result` 型が持つ便利なメソッドです。
    *   `find_phi_end` が返す `Result<f64, &'static str>` が `Err` だった場合に、そのエラーの種類を変換します。
    *   ここでは、`solver` から返ってきた単純なエラーメッセージ（`&'static str`）を、より詳細な情報を含む `String` 型のエラーに変換しています。
    *   `|e| format!(...)` はエラー内容 `e` を受け取るクロージャです。
*   **`Ok((ns, r))`**:
    *   計算がすべて成功した場合、`ns` と `r` を**タプル** `(f64, f64)` にまとめて `Ok` で包んで返しています。

---

### 全体のまとめ

全体の流れをもう一度おさらいすると、

1.  `main` が `config.toml` を読み込み (`serde`)
2.  `models` が設定に基づき、具体的なモデルのオブジェクトを `Box<dyn Potential>` として生成（トレイトオブジェクト）
3.  `calculation` が `Box<dyn Potential>` を受け取り、計算を開始
4.  `calculation` は `solver` を呼び出す。このとき `solver` は `impl Trait` を使って高速な静的ディスパッチで数値計算を実行
5.  `calculation` は `cosmology` の関数を使い、最終的な `(ns, r)` を計算して `main` に返す
6.  `main` が結果をファイルに書き出す

となります。静的ディスパッチと動的ディスパッチが、それぞれの利点を活かす形で使い分けられているのがポイントですね。
