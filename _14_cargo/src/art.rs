//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug, Eq, PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug, Eq, PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    ///
    /// # Examples
    ///
    /// ```
    /// use _14_cargo::art::mix;
    /// use _14_cargo::art::PrimaryColor;
    /// use _14_cargo::art::SecondaryColor;
    ///
    /// assert!(mix(PrimaryColor::Red, PrimaryColor::Red).is_none());
    /// assert!(mix(PrimaryColor::Yellow, PrimaryColor::Yellow).is_none());
    /// assert!(mix(PrimaryColor::Blue, PrimaryColor::Blue).is_none());
    ///
    /// assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Yellow).unwrap(), SecondaryColor::Orange);
    /// assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Blue).unwrap(), SecondaryColor::Purple);
    /// assert_eq!(mix(PrimaryColor::Yellow, PrimaryColor::Red).unwrap(), SecondaryColor::Orange);
    /// assert_eq!(mix(PrimaryColor::Yellow, PrimaryColor::Blue).unwrap(), SecondaryColor::Green);
    /// assert_eq!(mix(PrimaryColor::Blue, PrimaryColor::Red).unwrap(), SecondaryColor::Purple);
    /// assert_eq!(mix(PrimaryColor::Blue, PrimaryColor::Yellow).unwrap(), SecondaryColor::Green);
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> Option<SecondaryColor> {
        match c1 {
            PrimaryColor::Red => match c2 {
                PrimaryColor::Red => None,
                PrimaryColor::Yellow => Some(SecondaryColor::Orange),
                PrimaryColor::Blue => Some(SecondaryColor::Purple),
            },
            PrimaryColor::Yellow => match c2 {
                PrimaryColor::Red => Some(SecondaryColor::Orange),
                PrimaryColor::Yellow => None,
                PrimaryColor::Blue => Some(SecondaryColor::Green),
            },
            PrimaryColor::Blue => match c2 {
                PrimaryColor::Red => Some(SecondaryColor::Purple),
                PrimaryColor::Yellow => Some(SecondaryColor::Green),
                PrimaryColor::Blue => None,
            },
        }
    }
}
