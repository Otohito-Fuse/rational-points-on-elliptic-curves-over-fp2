const P: u64 = 7; // ここを適宜書き換える。2,3以外の素数で、かつ4で割って3余るものである必要がある。

pub mod complexification;
pub mod identities;
pub mod inverse;
pub mod modint;
pub mod polynomial;
pub mod solution_set;

use crate::complexification::Complex;
use crate::identities::{Identity, Zero};
use crate::modint::ModInt;
use crate::polynomial::Polynomial;
use crate::solution_set::SolutionSet;

use std::collections::HashSet;

fn main() {
    println!(
        "F_p^2 (p = {}) での y^2 = x^3 + ax + b の形の方程式の解を求めます。",
        P
    );

    if !is_prime(P) {
        println!("注：{}は素数ではありません。", P);
    }
    if P == 2 || P == 3 {
        println!("注：p = 2,3 ではこの形の表式では楕円曲線を網羅できません。")
    }
    if P % 4 != 3 {
        println!("注：p = {} のとき、x^2 = -1 となる x が F_p に存在するため、F_p[x] / (x^2 + 1) は体にならず、このプログラムでは F_p^2 を扱うことはできません。", P)
    }

    println!("係数aを入力");

    // 係数a
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).ok();
    // usize型に変換
    let a: u64 = a.trim().parse().ok().unwrap();

    println!("係数bを入力");

    // 係数b
    let mut b = String::new();
    std::io::stdin().read_line(&mut b).ok();
    // usize型に変換
    let b: u64 = b.trim().parse().ok().unwrap();

    let v: Vec<Complex<ModInt<P>>> = vec![
        Complex::<ModInt<P>>::new(ModInt::<P>::new(b), ModInt::<P>::zero()),
        Complex::<ModInt<P>>::new(ModInt::<P>::new(a), ModInt::<P>::zero()),
        Complex::<ModInt<P>>::zero(),
        Complex::<ModInt<P>>::identity(),
    ];

    let f: Polynomial<Complex<ModInt<P>>> = Polynomial::new(&v);

    let w: Vec<Complex<ModInt<P>>> = vec![
        Complex::<ModInt<P>>::zero(),
        Complex::<ModInt<P>>::zero(),
        Complex::<ModInt<P>>::identity(),
    ];

    let g: Polynomial<Complex<ModInt<P>>> = Polynomial::new(&w);

    let s: SolutionSet<(Complex<ModInt<P>>, Complex<ModInt<P>>)> = solve_equation(&f, &g);

    if ModInt::<P>::new(16)
        * (ModInt::<P>::new(4) * ModInt::<P>::new(a).modpow(3)
            + ModInt::<P>::new(27) * ModInt::<P>::new(b).modpow(2))
        == ModInt::<P>::zero()
    {
        println!("注：Δ = -16(4a^3 + 27b^2) = 0 なので方程式 {} = {} が定義する曲線は楕円曲線にはなりません。", g.print_f_of_y(),
        f.print_f_of_x(),);
    }

    println!(
        "方程式 {} = {} の解の集合は",
        g.print_f_of_y(),
        f.print_f_of_x(),
    );
    print_solutions(&s);
    println!("解の個数は");
    println!("{}個", s.size());
    println!("です。");
}

/// 素数判定
fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    if n == 0 || n == 1 {
        return false;
    }
    for i in 0..n {
        if n != 3 + 2 * i && n % (3 + 2 * i) == 0 {
            return false;
        }
        if (3 + 2 * i) * (3 + 2 * i) >= n {
            break;
        }
    }
    true
}

/// 方程式の解を全探索
fn solve_equation(
    f: &Polynomial<Complex<ModInt<P>>>,
    g: &Polynomial<Complex<ModInt<P>>>,
) -> SolutionSet<(Complex<ModInt<P>>, Complex<ModInt<P>>)> {
    let mut s: HashSet<(Complex<ModInt<P>>, Complex<ModInt<P>>)> = HashSet::new();
    for ir in 0..P {
        for ii in 0..P {
            for jr in 0..P {
                for ji in 0..P {
                    if Polynomial::evaluate(
                        &f,
                        Complex::<ModInt<P>>::new(ModInt::<P>::new(ir), ModInt::<P>::new(ii)),
                    ) == Polynomial::evaluate(
                        &g,
                        Complex::<ModInt<P>>::new(ModInt::<P>::new(jr), ModInt::<P>::new(ji)),
                    ) {
                        s.insert((
                            Complex::<ModInt<P>>::new(ModInt::<P>::new(ir), ModInt::<P>::new(ii)),
                            Complex::<ModInt<P>>::new(ModInt::<P>::new(jr), ModInt::<P>::new(ji)),
                        ));
                    }
                }
            }
        }
    }
    SolutionSet::new(s)
}

fn print_solutions(ss: &SolutionSet<(Complex<ModInt<P>>, Complex<ModInt<P>>)>) {
    let mut s: String = String::new();
    if ss.size() == 0 {
        s.push_str(&"{ }");
    } else {
        s.push_str(&"{");
        for (x, y) in &ss.unwrap() {
            s.push_str(&"(");
            s.push_str(&format!("{}", x).to_string());
            s.push_str(&", ");
            s.push_str(&format!("{}", y).to_string());
            s.push_str(&")");
            s.push_str(&", ");
        }
        s.pop();
        s.pop();
        s.push_str(&"}");
    }
    println!("{}", s);
}
