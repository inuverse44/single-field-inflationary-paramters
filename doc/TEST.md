# ユニットテストの雛形

Rustのユニットテストは、テスト対象のコードと同じファイルに記述するのが一般的です。これにより、プライベートな関数にもアクセスしやすくなります。

以下は `src/solver.rs` に追記するテストモジュールの雛形です。まずは `find_phi_end` 関数の正常系テストを実装してみましょう。

---

## `src/solver.rs` に追記するコード

ファイルの末尾に、以下のコードをコピー＆ペーストしてください。

```rust
// この行以下を src/solver.rs の末尾に追記

#[cfg(test)]
mod tests {
    use super::*; // 親モジュール（solver）のアイテムをすべてインポート
    use crate::potential::{ChaoticPotential};

    #[test]
    fn test_find_phi_end_chaotic_potential_success() {
        // --- 準備 (Arrange) ---
        // テスト対象のポテンシャルと計算精度を設定
        let potential = ChaoticPotential { m: 1.0, power: 2.0 };
        let precision = 0.000001;
        // ChaoticPotential(m=1)の場合、epsilon(phi)=1の解は phi = sqrt(2) になる
        let expected_phi_end = 2.0_f64.sqrt();
        // 解を十分に挟む探索範囲を設定
        let search_range = (1.0, 2.0);

        // --- 実行 (Act) ---
        // テスト対象の関数を呼び出す
        let result = find_phi_end(&potential, search_range, precision);

        // --- 検証 (Assert) ---
        // 実行結果が期待通りか検証する
        // `unwrap()` は `Result` が `Ok` であることを期待し、中身を取り出す。
        // もし `Err` ならテストはパニックして失敗する。
        let actual_phi_end = result.unwrap();

        // 浮動小数点数の比較では、完全一致 `==` ではなく、
        // 差が微小な値（イプシロン）以下であるかをチェックするのが定石。
        assert!((actual_phi_end - expected_phi_end).abs() < precision);
    }
}
```

### 解説

*   `#[cfg(test)]`: この属性がついたモジュールは、`cargo test` を実行したときにのみコンパイル・実行されます。
*   `mod tests { ... }`: テストコードをまとめるための慣習的なモジュール名です。
*   `use super::*;`: `tests`モジュールの親である`solver`モジュール内のすべてのアイテム（`find_phi_end`など）を使えるようにします。
*   `#[test]`: この属性をつけた関数が、テストとして実行されます。
*   **AAAパターン**: テストは「準備(Arrange)」「実行(Act)」「検証(Assert)」の3ステップで構成すると分かりやすくなります。
*   `assert!(...)`: `()`の中が`true`であることを検証します。`false`だとテストは失敗します。

### テストの実行方法

このコードを追記した後、ターミナルで以下のコマンドを実行すると、プロジェクト全体のテストが実行されます。

```bash
cargo test
```

---