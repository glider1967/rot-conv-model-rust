use std::marker::PhantomData;

use crate::constants::*;
use nalgebra::{SVector, SMatrix};

/// 両側粘着条件
struct DD;

/// ルジャンドル多項式でのスペクトル
struct LegendreSpectre<const D: usize>([f64; D]);

/// 緯度のスペクトル
struct LatSpectre(LegendreSpectre<{ LATITUDE_WAVE_NUM + 1 }>);

/// $\psi$ のスペクトル
struct PsiSpectre<BC>(
    LegendreSpectre<{ RADIAL_WAVE_NUM + 1 }>,
    PhantomData<BC>,
);
/// $\zeta$ のスペクトル
struct ZetaSpectre<BC>(
    LegendreSpectre<{ RADIAL_WAVE_NUM + 1 }>,
    PhantomData<BC>,
);
/// $\xi$ のスペクトル
struct XiSpectre(LegendreSpectre<{ RADIAL_WAVE_NUM + 1 }>);

/// 動径のスペクトル(ルジャンドル)
struct RadSpectre(LegendreSpectre<{ RADIAL_WAVE_NUM + 1 }>);

struct LegendreGrid<const D: usize>(SVector<f64, D>);

struct LatGrid(LegendreGrid<{ LATITUDE_GRID_NUM }>);
struct RadGrid(LegendreGrid<{ RADIAL_GRID_NUM }>);

struct Spectre([[f64; LATITUDE_WAVE_NUM + 1]; RADIAL_WAVE_NUM + 1]);
struct Grid([[f64; LATITUDE_GRID_NUM + 1]; RADIAL_GRID_NUM + 1]);

impl<const M: usize, const N: usize> From<LegendreSpectre<M>> for LegendreGrid<N> {
    fn from(_: LegendreSpectre<M>) -> Self {
        // TODO: spectre to grid transform
        unimplemented!()
    }
}

impl<const M: usize, const N: usize> From<LegendreGrid<N>> for LegendreSpectre<M> {
    fn from(_: LegendreGrid<N>) -> Self {
        // TODO: grid to spectre transform
        unimplemented!()
    }
}

impl From<PsiSpectre<DD>> for RadSpectre {
    fn from(psi: PsiSpectre<DD>) -> Self {
        let psi = psi.0.0;
        let mut legendre = [0.0; RADIAL_WAVE_NUM+1];
        for i in 1..RADIAL_WAVE_NUM-1 {
            let n = i as f64;
            legendre[i-1] += psi[i]/((4.0*n*n-1.0).sqrt());
            legendre[i+1] -= psi[i]/((4.0*n*n+5.0*n+3.0).sqrt());
        }
        RadSpectre(LegendreSpectre(legendre))
    }
}

impl From<RadSpectre> for PsiSpectre<DD> {
    fn from(_: RadSpectre) -> Self {
        unimplemented!()
    }
}

impl From<ZetaSpectre<DD>> for RadSpectre {
    fn from(_: ZetaSpectre<DD>) -> Self {
        unimplemented!()
    }
}

impl From<RadSpectre> for ZetaSpectre<DD> {
    fn from(_: RadSpectre) -> Self {
        unimplemented!()
    }
}

impl From<XiSpectre> for RadSpectre {
    fn from(_: XiSpectre) -> Self {
        unimplemented!()
    }
}

impl From<RadSpectre> for XiSpectre {
    fn from(_: RadSpectre) -> Self {
        unimplemented!()
    }
}

impl Spectre {
    fn lat_spectre(&self, l:usize) -> LatSpectre {
        unimplemented!()
    }

    fn rad_spectre(&self, n:usize) -> RadSpectre {
        unimplemented!()
    }
}

impl Grid {
    fn lat_grid(&self) -> LatGrid {
        unimplemented!()
    }

    fn rad_grid(&self) -> RadGrid {
        unimplemented!()
    }
}

impl From<Spectre> for Grid {
    fn from(_: Spectre) -> Self {
        unimplemented!()
    }
}

impl From<Grid> for Spectre {
    fn from(_: Grid) -> Self {
        unimplemented!()
    }
}