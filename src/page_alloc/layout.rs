#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    K4 = 1 << 12,
    K8 = 1 << 13,
    K16 = 1 << 14,
    K32 = 1 << 15,
    K64 = 1 << 16,
    K128 = 1 << 17,
    K256 = 1 << 18,
    K512 = 1 << 19,
    M1 = 1 << 20,
    M2 = 1 << 21,
    M4 = 1 << 22,
    M8 = 1 << 23,
    M16 = 1 << 24,
    M32 = 1 << 25,
    M64 = 1 << 26,
    M128 = 1 << 27,
    M256 = 1 << 28,
    M512 = 1 << 29,
    G1 = 1 << 30,
}

impl Align {
    #[inline]
    pub const fn as_power(&self) -> u32 {
        (*self as usize).trailing_zeros()
    }

    pub const fn from_power(power: u32) -> Option<Self> {
        match power {
            12 => Some(Self::K4),
            13 => Some(Self::K8),
            14 => Some(Self::K16),
            15 => Some(Self::K32),
            16 => Some(Self::K64),
            17 => Some(Self::K128),
            18 => Some(Self::K256),
            19 => Some(Self::K512),
            20 => Some(Self::M1),
            21 => Some(Self::M2),
            22 => Some(Self::M4),
            23 => Some(Self::M8),
            24 => Some(Self::M16),
            25 => Some(Self::M32),
            26 => Some(Self::M64),
            27 => Some(Self::M128),
            28 => Some(Self::M256),
            29 => Some(Self::M512),
            30 => Some(Self::G1),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageLayout {
    size: usize,
    align: Align,
}

impl PageLayout {
    pub const fn from_size(size: usize) -> Option<Self> {
        if size == 0 {
            return None;
        }
        if size < Align::K4 as usize {
            return Some(Self {
                size,
                align: Align::K4,
            });
        }
        let power = usize::BITS - size.leading_zeros() - 1;
        let align = if 1 << power == size {
            Align::from_power(power)
        } else {
            Align::from_power(power + 1)
        };
        if align.is_none() {
            None
        } else {
            Some(Self {
                size: size,
                align: align.unwrap(),
            })
        }
    }

    #[inline]
    pub const fn from_align(algin: Align) -> Self {
        Self {
            size: algin as usize,
            align: algin,
        }
    }

    #[inline]
    pub const fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub const fn align(&self) -> Align {
        self.align
    }

    #[inline]
    pub const fn align_to(&mut self, algin: Align) {
        self.align = algin;
    }
}

impl Default for PageLayout {
    fn default() -> Self {
        let align = Align::KB4;
        Self {
            size: align as usize,
            align,
        }
    }
}
