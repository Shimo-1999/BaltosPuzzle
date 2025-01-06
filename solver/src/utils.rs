const MASK30: u64 = (1 << 30) - 1;
const MASK31: u64 = (1 << 31) - 1;
pub const MOD: u64 = (1 << 61) - 1;
pub const BASE: u64 = 1000000007;

pub fn mul(a: u64, b: u64) -> u64 {
    let au = a >> 31;
    let ad = a & MASK31;
    let bu = b >> 31;
    let bd = b & MASK31;
    let mid = ad * bu + au * bd;
    let midu = mid >> 30;
    let midd = mid & MASK30;
    au * bu * 2 + midu + (midd << 31) + ad * bd
}

pub fn modulo(x: u64) -> u64 {
    let xu = x >> 61;
    let xd = x & MOD;
    let mut res = xu + xd;
    if res >= MOD {
        res -= MOD;
    }
    res
}

/// Σ_i a_i * base^{n-1-i}
pub fn hash(a: &[u64], base: u64) -> u64 {
    let mut ret = 0;
    for &v in a.iter() {
        ret = modulo(mul(ret, base) + v);
    }
    ret
}

pub fn bases(base: u64, n: usize) -> Vec<u64> {
    let mut bases = vec![1; n];
    for i in (1..n).rev() {
        bases[i - 1] = modulo(mul(bases[i], base));
    }
    bases
}

pub fn change(hash: u64, bases: &[u64], i: usize, old: u64, new: u64) -> u64 {
    assert!(i < bases.len());
    let mut diff = MOD + new - old;
    if diff >= MOD {
        diff -= MOD;
    }
    modulo(hash + mul(bases[i], diff))
}
