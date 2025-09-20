# Rustコードリーディング解説 (3): `potential.rs` と `cosmology.rs`

これは、Geminiによる `single-field-inflationary-paramters` プロジェクトのソースコード解説の続きです。

---

### `src/potential.rs`

```rust
pub trait Potential {
    fn v(&self, phi: f64) -> f64;
    fn p(&self, phi: f64) -> f64;
    fn p2(&self, phi: f64) -> f64;
}

pub struct ChaoticPotential {
    pub v0: f64,
    pub power: f64,
}

impl Potential for ChaoticPotential {
    fn v(&self, phi: f64) -> f64 {
        self.v0 * phi.powf(self.power)
    }
    // ... p, p2の実装 ...
}
```

*   **`pub trait Potential { ... }`**:
    *   **トレイト**は、ある型が持つべき共通の振る舞い（メソッドのシグネチャ）を定義する仕組みです。他の言語の「インターフェース」に非常に近いです。
    *   ここでは「インフレーションポテンシャルと見なせるものは、ポテンシャル `v`、その1階微分 `p`、2階微分 `p2` の3つのメソッドを必ず持たなければならない」という**契約**を定義しています。
*   **`impl Potential for ChaoticPotential`**:
    *   これは「`ChaoticPotential`という構造体は、`Potential`トレイトの契約を守ります」という宣言です。
    *   この `impl` ブロックの中で、トレイトで定義された全てのメソッド（`v`, `p`, `p2`）を具体的に実装する必要があります。もし実装漏れがあれば、コンパイルエラーになります。
*   **`&self`**:
    *   メソッドの第一引数に現れる `&self` は、そのメソッドが構造体のどのインスタンスに対して呼び出されているかを示します。`self` はそのインスタンス自身を指します。
    *   `&` がついているので、インスタンスの**不変の参照（borrow）**を受け取ります。つまり、メソッド内で `self` のフィールド（`self.v0` など）を読み取ることはできますが、書き換えることはできません。もし書き換えたい場合は `&mut self` と書きます。

---

### `src/cosmology.rs`

```rust
use crate::potential::Potential;

// ... (hubble_parameter, epsilon, etaなどの関数) ...

pub fn epsilon(potential: &(impl Potential + ?Sized), phi: f64) -> f64 {
    0.5 * M_P.powi(2) * (potential.p(phi) / potential.v(phi)).powi(2)
}

// ... (テストコード) ...
#[cfg(test)]
mod tests {
    use super::*;
    use crate::potential::{ChaoticPotential};

    #[test]
    fn test_epsilon() {
        // ...
    }
}
```

*   **`potential: &(impl Potential + ?Sized)`**:
    *   これは関数の引数でトレイトを使う方法の一つで、**impl Trait**構文と呼ばれます。
    *   `impl Potential` は「`Potential`トレイトを実装している**何らかの**具体的な型」を意味します。
    *   `models.rs`で見た `Box<dyn Potential>`（動的ディスパッチ）とは異なり、こちらは**静的ディスパッチ**を行います。
        *   **静的ディスパッチ**: コンパイル時に、この関数を呼び出している箇所の具体的な型（例: `ChaoticPotential`）をコンパイラが特定し、その型専用の関数コードを生成します。これにより、実行時に関数のアドレスを調べるコストがなくなり、非常に高速に動作します。インライン化などの最適化も効きやすくなります。
        *   **動的ディスパッチ**: 実行時に、トレイトオブジェクトが指す具体的な型に応じたメソッドを探して呼び出します。柔軟性が高い反面、わずかなオーバーヘッドがあります。
    *   `?Sized`: これは少し高度なトピックですが、「この型はコンパイル時にサイズが分からなくても良い（`Sized`でなくても良い）」ことを示すマーカーです。これにより、この関数が `&ChaoticPotential` のような参照だけでなく、`&dyn Potential` のようなトレイトオブジェクトの参照も受け取れるようになり、柔軟性が増します。
*   **`#[cfg(test)]` と `mod tests`**:
    *   `#[cfg(test)]` は、**条件付きコンパイル**のためのアトリビュートです。
    *   このアトリビュートがついた `mod tests` ブロックは、`cargo test` コマンドを実行したとき**だけ**コンパイル・実行され、`cargo build` で通常のバイナリをビルドするときには無視されます。
    *   これにより、テストコードが最終的な成果物に含まれないようにしつつ、ソースコードと同じファイル内にテストを記述できます。
*   **`use super::*;`**:
    *   テストモジュール `tests` の中から、その親モジュールである `cosmology` の中のすべての公開アイテム（`epsilon`関数など）をインポートしています。テストを書く際によく使われるイディオムです。

---

`potential.rs` のトレイトと `cosmology.rs` の `impl Trait` は、Rustのゼロコスト抽象化（パフォーマンスを犠牲にすることなく高いレベルの抽象化を可能にする機能）の好例です。
