use std::{marker::PhantomData, ops::Deref};

use crate::constants::*;

/// 両側粘着条件
struct DD;

/// ルジャンドル多項式でのスペクトル
struct LegendreSpectre<const D: usize>([f64; D]);

impl<const D: usize> LegendreSpectre<D> {
    fn new(spectre: [f64; D]) -> Self {
        Self(spectre)
    }
}

impl<const D:usize> Deref for LegendreSpectre<D> {
    type Target = [f64; D];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// $\psi$ のスペクトル
struct PsiSpectre<BC>(LegendreSpectre<{ RADIAL_WAVE_NUM + 1 }>, PhantomData<BC>);

impl<BC> PsiSpectre<BC> {
    fn new(spectre: LegendreSpectre<{RADIAL_WAVE_NUM + 1}>) -> Self {
        Self(spectre, PhantomData)
    }
}

impl<BC> Deref for PsiSpectre<BC> {
    type Target = LegendreSpectre<{RADIAL_WAVE_NUM + 1}>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// $\zeta$ のスペクトル
struct ZetaSpectre<BC>(LegendreSpectre<{ RADIAL_WAVE_NUM + 1 }>, PhantomData<BC>);
/// $\xi$ のスペクトル
struct XiSpectre(LegendreSpectre<{ RADIAL_WAVE_NUM + 1 }>);

/// 動径のスペクトル(ルジャンドル)
struct RadSpectre(LegendreSpectre<{ RADIAL_WAVE_NUM + 1 }>);

struct LegendreGrid<const D: usize>([f64; D]);


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
        let psi = &psi;
        let mut legendre = [0.0; RADIAL_WAVE_NUM + 1];
        for i in 1..=RADIAL_WAVE_NUM - 1 {
            legendre[i - 1] += psi[i] * A_COEFF[i - 1];
            legendre[i + 1] -= psi[i] * A_COEFF[i];
        }
        RadSpectre(LegendreSpectre(legendre))
    }
}

impl From<RadSpectre> for PsiSpectre<DD> {
    fn from(rad: RadSpectre) -> Self {
        let rad = rad.0 .0;
        let mut psi = [0.0; RADIAL_WAVE_NUM + 1];
        for i in 1..=RADIAL_WAVE_NUM-1 {
            psi[i] = rad[i - 1] * A_COEFF[i - 1] - rad[i + 1] * A_COEFF[i];
        }
        PsiSpectre::new(LegendreSpectre::new(psi))
    }
}

impl From<ZetaSpectre<DD>> for RadSpectre {
    fn from(zeta: ZetaSpectre<DD>) -> Self {
        let zeta = zeta.0 .0;
        let mut legendre = [0.0; RADIAL_WAVE_NUM + 1];
        for i in 2..=RADIAL_WAVE_NUM - 2 {
            legendre[i - 2] -= zeta[i] * D_COEFF[i - 2];
            legendre[i] += zeta[i] * C_COEFF[i - 1];
            legendre[i + 2] -= zeta[i] * D_COEFF[i];
        }
        RadSpectre(LegendreSpectre(legendre))
    }
}

impl From<RadSpectre> for ZetaSpectre<DD> {
    fn from(_: RadSpectre) -> Self {
        unimplemented!()
    }
}

impl From<XiSpectre> for RadSpectre {
    fn from(xi: XiSpectre) -> Self {
        let xi = xi.0 .0;
        let mut legendre = [0.0; RADIAL_WAVE_NUM + 1];
        for i in 1..=RADIAL_WAVE_NUM - 1 {
            legendre[i - 1] += xi[i] * A_COEFF[i - 1];
            legendre[i + 1] -= xi[i] * A_COEFF[i];
        }
        RadSpectre(LegendreSpectre(legendre))
    }
}

impl From<RadSpectre> for XiSpectre {
    fn from(_: RadSpectre) -> Self {
        unimplemented!()
    }
}