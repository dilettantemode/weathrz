pub(crate) const SKY_TOP: u16 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkyBand {
    pub top: u16,
    pub bottom: u16,
}

impl SkyBand {
    pub fn below_hud(skyline_y: u16) -> Self {
        Self {
            top: SKY_TOP,
            bottom: skyline_y.max(SKY_TOP),
        }
    }

    pub fn height(&self) -> u16 {
        self.bottom - self.top
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_band_sits_between_the_hud_and_the_roof() {
        assert_eq!(SkyBand::below_hud(15), SkyBand { top: 3, bottom: 15 });
        assert_eq!(SkyBand::below_hud(15).height(), 12);
        assert_eq!(SkyBand::below_hud(2).height(), 0);
    }
}
