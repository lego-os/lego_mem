#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Align1Shl12 = 1 << 12,
    Align1Shl13 = 1 << 13,
    Align1Shl14 = 1 << 14,
    Align1Shl15 = 1 << 15,
    Align1Shl16 = 1 << 16,
    Align1Shl17 = 1 << 17,
    Align1Shl18 = 1 << 18,
    Align1Shl19 = 1 << 19,
    Align1Shl20 = 1 << 20,
    Align1Shl21 = 1 << 21,
    Align1Shl22 = 1 << 22,
    Align1Shl23 = 1 << 23,
    Align1Shl24 = 1 << 24,
    Align1Shl25 = 1 << 25,
    Align1Shl26 = 1 << 26,
    Align1Shl27 = 1 << 27,
    Align1Shl28 = 1 << 28,
    Align1Shl29 = 1 << 29,
    Align1Shl30 = 1 << 30,
}

impl Align {
    #[inline]
    pub const fn as_order(&self) -> u32 {
        (*self as usize).trailing_zeros()
    }

    pub const fn from_order(order: u32) -> Option<Self> {
        match order {
            12 => Some(Self::Align1Shl12),
            13 => Some(Self::Align1Shl13),
            14 => Some(Self::Align1Shl14),
            15 => Some(Self::Align1Shl15),
            16 => Some(Self::Align1Shl16),
            17 => Some(Self::Align1Shl17),
            18 => Some(Self::Align1Shl18),
            19 => Some(Self::Align1Shl19),
            20 => Some(Self::Align1Shl20),
            21 => Some(Self::Align1Shl21),
            22 => Some(Self::Align1Shl22),
            23 => Some(Self::Align1Shl23),
            24 => Some(Self::Align1Shl24),
            25 => Some(Self::Align1Shl25),
            26 => Some(Self::Align1Shl26),
            27 => Some(Self::Align1Shl27),
            28 => Some(Self::Align1Shl28),
            29 => Some(Self::Align1Shl29),
            30 => Some(Self::Align1Shl30),
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
        let order = usize::BITS - size.leading_zeros() - 1;
        let align = if 1 << order == size {
            Align::from_order(order)
        } else {
            Align::from_order(order + 1)
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
