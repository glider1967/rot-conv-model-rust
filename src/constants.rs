use once_cell::sync::Lazy;

/// 定数

pub const LATITUDE_GRID_NUM: usize = 16;
pub const RADIAL_GRID_NUM: usize = 10;
pub const LATITUDE_WAVE_NUM: usize = 8;
pub const RADIAL_WAVE_NUM: usize = 5;


pub static A_COEFF: Lazy<[f64; RADIAL_WAVE_NUM]> = Lazy::new(|| {
    let mut a = [0.0; RADIAL_WAVE_NUM];
    for n in 1..RADIAL_WAVE_NUM {
        let nf = n as f64;
        a[n-1] = 1.0/((4.0*nf*nf - 1.0).sqrt());
    }
    a
});
