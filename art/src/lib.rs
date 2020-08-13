//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::{PrimaryColor, SecondaryColor};
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // // lol
        // match (c1,c2){
        //     (PrimaryColor::Red, PrimaryColor::Red) => {}
        //     (PrimaryColor::Red, PrimaryColor::Yellow) => SecondaryColor::Green,
        //     (PrimaryColor::Red, PrimaryColor::Blue) => SecondaryColor::Purple,
        //     (PrimaryColor::Yellow, PrimaryColor::Red) =>
        //     (PrimaryColor::Yellow, PrimaryColor::Yellow) => {}
        //     (PrimaryColor::Yellow, PrimaryColor::Blue) => {}
        //     (PrimaryColor::Blue, PrimaryColor::Red) => {}
        //     (PrimaryColor::Blue, PrimaryColor::Yellow) => {}
        //     (PrimaryColor::Blue, PrimaryColor::Blue) => {}
        // }
        SecondaryColor::Purple
    }
}
