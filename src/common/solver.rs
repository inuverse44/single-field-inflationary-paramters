use crate::models::Potential;
use crate::cosmology::{epsilon};


// ε(φ) = 1 となる φ_endを見つける
pub fn find_phi_end(potential: &(impl Potential + ?Sized), search_range: (f64, f64), precision: f64) -> Result<f64, &'static str> {

    // 根が存在するかチェック
    let f = epsilon(potential, search_range.0) - 1.0;
    let g = epsilon(potential, search_range.1) - 1.0;
    if f * g >= 0.0 {
        return Err("Root could not be specified. Change phi1 and/or phi2.");
    }

    let mut phi_a = search_range.0;
    let mut phi_b = search_range.1;
    let mut phi_c = (phi_a + phi_b) / 2.0;          // 中心
    let mut fc = epsilon(potential, phi_c) - 1.0;    // その時の関数の値f(c) = ε(φ_c) - 1
    while fc.abs() < precision {
        if f * fc < 0.0 {
            phi_b = phi_c;
        } else {
            phi_a = phi_c;
        }
        phi_c = (phi_a + phi_b) / 2.0;              // 場の値の更新
        fc = epsilon(potential, phi_c) - 1.0;       // 関数の値の更新
    }
    Ok(phi_c)
}

// e-foldの積分計算を行う
/// e-foldの積分計算を行い、対応するφの値を返す
pub fn find_phi_exit(
    potential: &(impl Potential + ?Sized), 
    phi_end: f64, 
    n_target: f64, // e-foldの目標値
    search_range: (f64, f64),
    precision: f64,
    max_iter: usize,
) -> Result<f64, &'static str> {

    //　クロージャが使える
    let integrand = |phi: f64| potential.v(phi) / potential.p(phi);
    let efold_num = |phi: f64| simpson(|p| integrand(p), phi_end, phi, precision, max_iter);
    let find_root = |phi: f64| efold_num(phi) - n_target;

    let mut phi_a = search_range.0;
    let mut phi_b = search_range.1;
    println!("serch range: ({}, {}) and its diff: {}", 
            search_range.0, search_range.1, search_range.1 - search_range.0);

    // 根が存在するかチェック
    let fa = find_root(phi_a);
    let fb = find_root(phi_b);
    if fa * fb >= 0.0 {
        println!("fa: {}, fb: {}, fa * fb: {}",fa, fb,  fa * fb);
        return Err("Root for phi_exit not in search range. Change the range.");
    }

    let mut phi_c = (phi_a + phi_b) / 2.0;
    let mut fc = find_root(phi_c);

    while (phi_b - phi_a).abs() > precision {
        if fc.abs() < precision {
            return Ok(phi_c);
        }
        if fa * fc < 0.0 {
            phi_b = phi_c;
        } else {
            phi_a = phi_c;
        }
        phi_c = (phi_a + phi_b) / 2.0;
        fc = find_root(phi_c);
    }
    Ok(phi_c)
}

// TODO: 前ステップの分割したときの値を使いきれていない。まだ最適化できる。
pub fn simpson<F>(f: F, a: f64, b: f64, precision: f64, max_iter: usize) -> f64 
    where F: Fn(f64) -> f64 {

    let mut n = 2; // 初期分割数
    let mut last_result = simpson_internal(&f, a, b, n);

    for _ in 0..max_iter { // 念のため最大20回程度の反復に制限
        n *= 2; // 分割数を2倍にする
        let current_result = simpson_internal(&f, a, b, n);

        // 前回の結果との相対誤差を計算
        let error = (current_result - last_result).abs() / last_result.abs();

        if error < precision {
            return current_result; // 精度を満たしたらループを抜ける
        }

        last_result = current_result;
    }
    println!("Simpson's rule did not converge within the given iterations.");
    last_result // 収束しなかった場合は最後の結果を返す
}

/// シンプソン法の内部計算（固定分割数）
fn simpson_internal<F>(f: &F, a: f64, b: f64, n: usize) -> f64 
    where F: Fn(f64) -> f64 {
    let n = if n % 2 == 1 { n + 1} else { n };  // nを偶数
    let h = (b - a) / n as f64;                 // 微小幅

    let mut sum1 = 0.0;
    for i in (1..n).step_by(2) {
        sum1 += f(a + i as f64 * h);
    }

    let mut sum2 = 0.0;
    for i in (2..n).step_by(2) {
        sum2 += f(a + i as f64 * h);
    }

    h / 3.0 * (f(a) + f(b) + 4.0 * sum1 + 2.0 * sum2)
}


// AAA pattern (Arrange, Act, Assert)の順に従ってテストコードを組むとわかりやすい
#[cfg(test)]
mod tests {
    use super::*; // 親モジュール（solver）のアイテムをすべてインポート
    use crate::models::chaotic::potential::ChaoticPotential;

    #[test]
    fn test_find_phi_end_chaotic_potential_success() {
        // ----- Arrange -----
        // テスト対象のポテンシャルと計算精度を設定
        let potential = ChaoticPotential { v0: 1.0, power: 2.0 };
        let precision = 1.0e-6;
        let expected_phi_end = 2.0_f64.sqrt();
        // 解を十分に挟む探索範囲を設定
        let search_range = (1.0, 2.0);

        // ----- Act -----
        // テスト対象の関数を呼び出す
        let result = find_phi_end(&potential, search_range, precision);

        // ----- Assert -----
        // 実行結果が期待通りか検証する
        // `unwrap()` は `Result` が `Ok` であることを期待し、中身を取り出す。
        // もし `Err` ならテストはパニックして失敗する。
        let actual_phi_end = result.unwrap();

        // 浮動小数点数の比較では、完全一致 `==` ではなく、
        // 差が微小な値（イプシロン）以下であるかをチェックするのが定石。
        assert!((actual_phi_end - expected_phi_end).abs() < precision);
    }

    #[test]
    fn test_find_phi_exit_chaotic_potential_success() {
        // ----- Arange ----- 
        let potential = ChaoticPotential { v0: 1.0, power: 2.0 };
        let precision = 1.0e-6;
        let expected_efolds = 60.0;
        let power = 2.0;
        let phi_end = power / 2.0_f64;
        let expected_phi_exit = (2.0 * power * expected_efolds + phi_end ).sqrt(); // これは解析的に導かれる

        // ----- Act ----- 
        let result = find_phi_exit(&potential, phi_end, expected_efolds, (1.0, 30.0), precision, 20);
        let actual_phi_exit = result.unwrap();

        // ----- Assert -----
        assert!((actual_phi_exit - expected_phi_exit).abs() < precision);

    }

    #[test]
    fn test_simpson() {
        // ----- Arange -----
        // 被積分関数 f(x) = sin(x)
        let f = |x: f64| x.sin();
        let a = 0.0;
        let b = 1.0;
        let precision = 1.0e-9;

        // ∫_0^1 sin(x) dx = -cos(1) - (-cos(0)) = 1 - cos(1)
        let expected_answer = 1.0 - 1.0_f64.cos();

        // ----- Act -----
        let actual_answer = simpson(f, a, b, precision, 20);

        // ----- Assert -----
        assert!((actual_answer - expected_answer).abs() < 1.0e-7);
    }
        
}