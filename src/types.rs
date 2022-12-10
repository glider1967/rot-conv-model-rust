use std::marker::PhantomData;

use crate::constants;
use nalgebra::SVector;

/// 両側粘着条件
struct DD;

/// ルジャンドル多項式でのスペクトル
struct LegendreSpectre<const D: usize>(SVector<f64, D>);

/// 緯度のスペクトル
struct LatSpectre(LegendreSpectre<{ constants::LATITUDE_WAVE_NUM + 1 }>);

/// $\psi$ のスペクトル
struct PsiSpectre<BC>(
    LegendreSpectre<{ constants::RADIAL_WAVE_NUM + 1 }>,
    PhantomData<BC>,
);
/// $\zeta$ のスペクトル
struct ZetaSpectre<BC>(
    LegendreSpectre<{ constants::RADIAL_WAVE_NUM + 1 }>,
    PhantomData<BC>,
);
/// $\xi$ のスペクトル
struct XiSpectre(LegendreSpectre<{ constants::RADIAL_WAVE_NUM + 1 }>);

/// 動径のスペクトル(ルジャンドル)
struct RadSpectre(LegendreSpectre<{ constants::RADIAL_WAVE_NUM + 1 }>);

struct LegendreGrid<const D: usize>(SVector<f64, D>);

struct LatGrid(LegendreGrid<{ constants::LATITUDE_GRID_NUM }>);
struct RadGrid(LegendreGrid<{ constants::RADIAL_GRID_NUM }>);

impl<const M: usize, const N: usize> From<LegendreSpectre<M>> for LegendreGrid<N> {
    fn from(_: LegendreSpectre<M>) -> Self {
        // TODO: spectre to grid transform
        Self(SVector::zeros())
    }
}

impl<const M: usize, const N: usize> From<LegendreGrid<N>> for LegendreSpectre<M> {
    fn from(_: LegendreGrid<N>) -> Self {
        // TODO: grid to spectre transform
        Self(SVector::zeros())
    }
}

impl From<PsiSpectre<DD>> for RadSpectre {
    fn from(_: PsiSpectre<DD>) -> Self {
        Self(LegendreSpectre(SVector::zeros()))
    }
}

impl From<RadSpectre> for PsiSpectre<DD> {
    fn from(_: RadSpectre) -> Self {
        Self(LegendreSpectre(SVector::zeros()), PhantomData)
    }
}

impl From<ZetaSpectre<DD>> for RadSpectre {
    fn from(_: ZetaSpectre<DD>) -> Self {
        Self(LegendreSpectre(SVector::zeros()))
    }
}

impl From<RadSpectre> for ZetaSpectre<DD> {
    fn from(_: RadSpectre) -> Self {
        Self(LegendreSpectre(SVector::zeros()), PhantomData)
    }
}

impl From<XiSpectre> for RadSpectre {
    fn from(_: XiSpectre) -> Self {
        Self(LegendreSpectre(SVector::zeros()))
    }
}

impl From<RadSpectre> for XiSpectre {
    fn from(_: RadSpectre) -> Self {
        Self(LegendreSpectre(SVector::zeros()))
    }
}
