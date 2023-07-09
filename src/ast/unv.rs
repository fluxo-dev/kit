//! Stratified type universe.

use crate::err::SystemErr;
use crate::fmt::Formatted;
use std::fmt::{Display, Formatter};

/// Universe, aka *sort*, which refers to a space where types exist.
///
/// In principle, there are an infinite number of universes, with each universe assigned a distinct
/// natural number starting with 0. For practical reasons, levels higher than [u64::MAX] will cause
/// an [SystemErr][crate::err::SystemErr]. Universes are cumulative: a type that belongs to any
/// given level 'N' automatically belongs to universes at higher levels.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Unv {
    /// Level assigned to the universe.
    pub level: u64,
}

impl Unv {
    /// Create a new instance of a universe at level 0.
    pub fn new() -> Self {
        Self { level: 0 }
    }

    /// Create a new universe at a level higher that the current.
    pub fn inc(&self) -> Result<Self, SystemErr> {
        self.level
            .checked_add(1)
            .map(|level| Self { level })
            .ok_or(SystemErr::MaxLimitUnv(self.level))
    }
}

impl Default for Unv {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Unv {
    fn fmt(&self, f: &mut Formatter<'_>) -> Formatted {
        write!(f, "□")
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_inc() {
        let o1 = Unv::new();
        assert_eq!(o1.level, 0);
        assert!(o1.inc().is_ok());
        assert_eq!(o1.inc().unwrap().level, 1);
    }

    #[test]
    fn test_inc_overflow() {
        let o1 = Unv {
            level: u64::MAX - 1,
        };
        assert!(o1.inc().is_ok());
        assert_eq!(o1.inc().unwrap().level, u64::MAX);

        let o2 = o1.inc().unwrap();
        assert!(o2.inc().is_err()); // overflow expected
    }

    #[test]
    fn test_max() {
        let o1 = Unv::new();
        let o2 = Unv::new().inc().unwrap();
        let o3 = Unv::new().inc().unwrap().inc().unwrap();
        assert_eq!(Unv::max(o1, o2), o2);
        assert_eq!(Unv::max(o2, o1), o2);
        assert_eq!(Unv::max(o2, o3), o3);
        assert_eq!(Unv::max(o3, o2), o3);
    }

    #[test]
    fn test_display() -> Result<(), SystemErr> {
        let o1 = Unv::new();
        let o2 = o1.inc()?;
        let o3 = o2.inc()?;
        let o4 = Unv { level: 3944 };
        assert_eq!(o1.to_string(), "□");
        assert_eq!(o2.to_string(), "□");
        assert_eq!(o3.to_string(), "□");
        assert_eq!(o4.to_string(), "□");
        Ok(())
    }
}
