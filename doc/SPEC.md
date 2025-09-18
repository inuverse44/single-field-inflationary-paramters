### **仕様書: $n_s-r$ 図作成のためのアーキテクチャ改善**

**1. 目的**

- 現在の `main.rs` は、特定のモデル（Chaotic inflation）と特定のパラメータ（`power: 2.0`）に固定されており、$n_s-r$図の軌跡を描画するような、パラメータを動かした繰り返し計算に対応できていない

- 様々なインフレーションモデルとそのモデルパラメータに対して $n_s-r$ の軌跡を計算し、プロット用のデータファイルを生成するための、柔軟で拡張性の高いアーキテクチャを考える

**2. 設計方針**

- **関心の分離**
    - **設定:** 何を計算するか（モデル、パラメータ範囲など）
    - **実行:** 設定に基づいて計算を繰り返し実行
    - **計算:** 1組のパラメータに対する $(n_s, r)$ の計算ロジック
    - **出力:** 計算結果をファイルに書き出す
    これらを明確に分離し、それぞれ独立して変更・拡張可能

- **設定の外部化**
    - 計算対象のモデルやパラメータをソースコードから分離し、設定ファイル（例: `config.toml`）で管理
    - これにより、再コンパイルすることなく計算条件を変更可能

- **抽象化による拡張性**
    - `potential.rs` の `Potential` トレイトを活かし、新しいインフレーションモデルを簡単に追加できる構造を構築

- **構造化された出力**
    - 計算結果をCSV形式で出力し、Python (Matplotlib, Seaborn) や Gnuplot などの外部ツールで容易にグラフ化できるようにします。

**3. アーキテクチャの構成要素**

**3.1. 設定レイヤー (`config.rs`, `config.toml`)**

計算の挙動をTOMLファイルで定義し、`serde`クレートを使ってRustの構造体に読み込む。

-   **`config.toml` (記述例):**
    ```toml
    # 出力先のファイルパス
    output_file = "output/ns_r_diagram.csv"

    # --- 計算するシミュレーションのリスト ---
    [[simulations]]
    # 凡例などで使うシミュレーション名
    name = "Chaotic (p-scan)"
    # 使用するモデルの種類
    model = "Chaotic"
    # ループで動かすパラメータ ("parameter_scan")
    scan_parameter = { name = "power", start = 1.0, end = 4.0, steps = 50 }
    # 固定するパラメータ ("fixed_parameters")
    fixed_parameters = { m = 0.01 }

    [[simulations]]
    name = "Chaotic (m-scan)"
    model = "Chaotic"
    scan_parameter = { name = "m", start = 0.001, end = 0.1, steps = 50 }
    fixed_parameters = { power = 2.0 }

    # [[simulations]]
    # name = "NewInflationModel"
    # model = "NewModel"
    # ... (将来のモデル追加)
    ```

-   **`src/config.rs` (対応する構造体):**
    ```rust
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Deserialize)]
    pub struct Config {
        pub output_file: String,
        pub simulations: Vec<Simulation>,
    }

    #[derive(Deserialize)]
    pub struct Simulation {
        pub name: String,
        pub model: String,
        pub scan_parameter: Scan,
        pub fixed_parameters: HashMap<String, f64>,
    }

    #[derive(Deserialize)]
    pub struct Scan {
        pub name: String,
        pub start: f64,
        pub end: f64,
        pub steps: usize,
    }
    ```

**3.2. モデル定義レイヤー (`models.rs`)**

設定ファイルの情報から、適切な `Potential` オブジェクトを生成する役割を担います。

-   **ファクトリ (Factory) パターン:**
    -   `model`名とパラメータの`HashMap`を受け取り、対応する `Box<dyn Potential>` を返す関数（またはメソッド）を作成します。これにより、`main.rs` は具体的なモデル構造体を知る必要がなくなります。

-   **`src/models.rs` (実装イメージ):**
    ```rust
    use crate::potential::{Potential, ChaoticPotential};
    use std::collections::HashMap;

    // HashMapのパラメータからChaoticPotentialを生成する
    fn create_chaotic(params: &HashMap<String, f64>) -> Result<ChaoticPotential, String> {
        let m = *params.get("m").ok_or("parameter 'm' is missing")?;
        let power = *params.get("power").ok_or("parameter 'power' is missing")?;
        // 注: この実装は、ChaoticPotentialのprime/double_primeが
        //     powerを正しく使うように修正されることを前提とします。
        Ok(ChaoticPotential { m, power })
    }

    // 文字列とパラメータから、対応するPotentialトレイトオブジェクトを生成する
    pub fn create_potential(model_name: &str, params: &HashMap<String, f64>) -> Result<Box<dyn Potential>, String> {
        match model_name {
            "Chaotic" => {
                let potential = create_chaotic(params)?;
                Ok(Box::new(potential))
            },
            // "NewModel" => { ... } // 新規モデルはここに追加
            _ => Err(format!("Unknown model: {}", model_name)),
        }
    }
    ```

**3.3. 計算ロジックレイヤー (`calculation.rs`)**

現在 `main.rs` にある `(ns, r)` の計算ロジックを、独立した関数に切り出します。

-   **`src/calculation.rs` (関数シグネチャ):**
    ```rust
    use crate::potential::Potential;

    pub fn calculate_ns_r(
        potential: &dyn Potential,
        n_target: f64,
        precision: f64,
    ) -> Result<(f64, f64), String> {
        // 1. find_phi_end を呼び出す
        // 2. find_phi_exit を呼び出す
        // 3. epsilon, eta を計算する
        // 4. spectral_index, tensor_to_scalar_ratio を計算する
        // 5. Ok((ns, r)) を返す
        // ※途中でエラーが発生した場合は、Err(エラーメッセージ)を返す
    }
    ```

**3.4. 実行レイヤー (`main.rs`)**

`main`関数は、上記コンポーネントを組み合わせて全体の流れを制御する「オーケストレーター」に徹します。

-   **`main`関数の処理フロー:**
    1.  `config.toml` を読み込む (`config.rs`)。
    2.  出力用CSVファイルを作成し、ヘッダー行 (`simulation_name, parameter_name, parameter_value, ns, r`) を書き込む。
    3.  設定された `simulations` をループする。
    4.  各シミュレーションの `scan_parameter` に従って、パラメータをループする。
    5.  ループ内で、`fixed_parameters` と現在のスキャンパラメータを結合する。
    6.  `models::create_potential` を呼び出し、`Potential` オブジェクトを生成する。
    7.  `calculation::calculate_ns_r` を呼び出し、`(ns, r)` を計算する。
    8.  結果をCSVファイルに追記する。
    9.  進捗がわかるように、コンソールにログを出力する。

---

このアーキテクチャを採用することで、コードの役割分担が明確になり、今後の機能追加（新しいインフレーションモデルなど）やメンテナンスが非常に容易になります。
