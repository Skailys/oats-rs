//! This crate provides a simple and efficient way to generate globally unique identifiers.

// This feature is needed to get the number of leading zeros in a number.
#![feature(core_intrinsics)]

/// The bowl is used for generating Oat values in unified way.
pub mod bowl;

/// The oats are globally unique identifiers.
pub mod oat;

/// The region moduele contains predefined regions
pub mod region;

#[cfg(test)]
mod tests {
    use super::*;

    mod bowl {
        use crate::bowl::{Bowl, GenerationBehavior, WrappedBowl};
        use crate::oat::Oat;

        #[test]
        fn test_wrapped_bowl_generate() {
            // Create a WrappedBowl instance with node id 1 and normal generation behavior
            let wrapped_bowl: WrappedBowl = WrappedBowl::of(1, GenerationBehavior::Normal, None);
            test_wrapped_bowl_generate_definition(wrapped_bowl)
        }

        #[test]
        fn test_wrapped_bowl_generate_lazy() {
            // Create a WrappedBowl instance with node id 1 and lazy generation behavior
            let wrapped_bowl: WrappedBowl = WrappedBowl::of(1, GenerationBehavior::Lazy, None);
            test_wrapped_bowl_generate_definition(wrapped_bowl)
        }

        #[test]
        fn test_bowl_new_seq_realtime() {
            let mut bowl: Bowl = Bowl::of(1, GenerationBehavior::Realtime, None);

            // Generate 10 Oat values with the Realtime generation behavior mode
            let oats: Vec<u16> = (0..10).map(|_| bowl.new_seq()).collect();

            // Check that the sequence numbers of the Oat values are increasing
            assert!(oats.windows(2).all(|window| window[0] < window[1]));
        }

        fn test_wrapped_bowl_generate_definition(wrapped_bowl: WrappedBowl) {
            // Generate 10 Oat values
            let oats: Vec<Oat> = (0..4095).map(|_| wrapped_bowl.generate()).collect();

            // Check that the Oat values have the correct node id
            assert!(oats.iter().all(|oat| oat.node() == 1));

            // Check that the Oat values have increasing sequence numbers
            assert!(oats.windows(2).all(|window| window[0].seq() < window[1].seq()));

            // Check if seq restarts
            let oat = wrapped_bowl.generate();
            assert_eq!(oat.seq(), 0);
            assert_eq!(oat.node(), 1);
            assert!(oat.timestamp() > oats.last().unwrap().timestamp())
        }
    }

    mod oat {
        use crate::oat::Oat;

        #[test]
        fn test_to_string() {
            let oat = Oat::of(1, 3, 1671800400_000);
            let s = oat.to_string();

            assert_eq!(s, "X1AwCIGvFTGAA");
        }
    }
}
