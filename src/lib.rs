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
    /// This module contains the implementation of the `bowl` module.
    mod bowl {
        use std::thread;

        use crate::bowl::{Bowl, GenerationBehavior, WrappedBowl};
        use crate::oat::Oat;

        /// Test function for generating Oat values using a WrappedBowl with normal generation behavior.
        #[test]
        fn test_wrapped_bowl_generate() {
            // Create a WrappedBowl instance with node id 1 and normal generation behavior
            let wrapped_bowl: WrappedBowl = WrappedBowl::of(1, GenerationBehavior::Normal, None);
            test_wrapped_bowl_generate_definition(wrapped_bowl)
        }

        /// Test function for generating Oat values using a WrappedBowl with lazy generation behavior.
        #[test]
        fn test_wrapped_bowl_generate_lazy() {
            // Create a WrappedBowl instance with node id 1 and lazy generation behavior
            let wrapped_bowl: WrappedBowl = WrappedBowl::of(1, GenerationBehavior::Lazy, None);
            test_wrapped_bowl_generate_definition(wrapped_bowl)
        }

        /// Test function for creating a Bowl with Realtime generation behavior and generating Oat values.
        #[test]
        fn test_bowl_new_seq_realtime() {
            let mut bowl: Bowl = Bowl::of(1, GenerationBehavior::Realtime, None);

            // Generate 10 Oat values with the Realtime generation behavior mode
            let oats: Vec<u16> = (0..10).map(|_| bowl.new_seq()).collect();

            // Check that the sequence numbers of the Oat values are increasing
            assert!(oats.windows(2).all(|window| window[0] < window[1]));
        }

        /// Test function for generating Oat values using a WrappedBowl in a multi-threaded environment.
        #[test]
        fn test_wrapped_bowl_in_multi_thread_env() {
            let wrapped: WrappedBowl = WrappedBowl::of(1, GenerationBehavior::Normal, None);
            let mut handles = vec![];

            for _ in 0..10 {
                let clone = wrapped.clone();
                handles.push(thread::spawn(move || {
                    clone.generate();
                }));
            }

            handles.into_iter().for_each(|h| h.join().unwrap());
        }

        /// Test function for generating Oat values using a WrappedBowl.
        fn test_wrapped_bowl_generate_definition(wrapped_bowl: WrappedBowl) {
            // Generate 10 Oat values
            let oats: Vec<Oat> = (0..4095).map(|_| wrapped_bowl.generate()).collect();

            // Check that the Oat values have the correct node id
            assert!(oats.iter().all(|oat| oat.node() == 1));

            // Check that the Oat values have increasing sequence numbers
            assert!(oats
                .windows(2)
                .all(|window| window[0].seq() < window[1].seq()));

            // Check if seq restarts
            let oat = wrapped_bowl.generate();
            assert_eq!(oat.seq(), 0);
            assert_eq!(oat.node(), 1);
            assert!(oat.timestamp() > oats.last().unwrap().timestamp())
        }
    }

    /// The `oat` module contains tests for the `Oat` struct.
    mod oat {
        use crate::oat::Oat;

        /// Test the `to_string` method of the `Oat` struct.
        #[test]
        fn test_to_string() {
            let oat = Oat::of(1, 3, 1671800400_000);
            let s = oat.to_string();

            assert_eq!(s, "X1AwCIGvFTGAA");
        }

        /// Test the `to_bytes` method of the `Oat` struct.
        #[test]
        fn test_to_bytes() {
            let oat = Oat::of(1, 3, 1671800400_000);
            let b: [u8; 9] = oat.to_bytes();

            assert_eq!(b, [1, 0x03, 0x0, 0x88, 0x1A, 0xF1, 0x53, 0x18, 0x0]);
        }

        /// Test the `into` method of the `Oat` struct.
        #[test]
        fn test_into_bytes() {
            let oat = Oat::of(1, 3, 1671800400_000);
            let b: [u8; 9] = oat.into();

            assert_eq!(b, [1, 0x03, 0x0, 0x88, 0x1A, 0xF1, 0x53, 0x18, 0x0]);
        }

        /// Test the `from_string` method of the `Oat` struct.
        #[test]
        fn test_from_string() {
            let oat = Oat::from_string("X1AwCIGvFTGAA").unwrap();

            assert_eq!(oat.node(), 1);
            assert_eq!(oat.seq(), 3);
            assert_eq!(oat.timestamp(), 1671800400_000);
        }

        /// Test the `from_string_unchecked` method of the `Oat` struct.
        #[test]
        fn test_from_string_unchecked() {
            let oat = Oat::from_string_unchecked("X1AwCIGvFTGAA");

            assert_eq!(oat.node(), 1);
            assert_eq!(oat.seq(), 3);
            assert_eq!(oat.timestamp(), 1671800400_000);
        }

        /// Test the `from_bytes` method of the `Oat` struct.
        #[test]
        fn test_from_bytes() {
            let oat = Oat::from_bytes([1, 0x03, 0x0, 0x88, 0x1A, 0xF1, 0x53, 0x18, 0x0]);

            assert_eq!(oat.node(), 1);
            assert_eq!(oat.seq(), 3);
            assert_eq!(oat.timestamp(), 1671800400_000);
        }

        /// Test the `from_bytes_ref` method of the `Oat` struct.
        #[test]
        fn test_from_bytes_ref() {
            let oat =
                Oat::from_bytes_ref(&[1, 0x03, 0x0, 0x88, 0x1A, 0xF1, 0x53, 0x18, 0x0]).unwrap();

            assert_eq!(oat.node(), 1);
            assert_eq!(oat.seq(), 3);
            assert_eq!(oat.timestamp(), 1671800400_000);
        }
    }
}
