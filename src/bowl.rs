use crate::oat::Oat;
use std::{hint::spin_loop, sync::{Arc, Mutex}, time::SystemTime};

/// Defines the behavior of generating new Oats
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum GenerationBehavior {
    /// This option takes the current timestamp and increments seq id until it overflows and adds one to the timestamp.
    Lazy,
    /// This is the default mode, it maintains the current timestamp like lazy until there are no free seq id's left, then it syncs the timestamp to the current time.
    Normal,
    /// As long as there is no significant time change, the seq id will be incremented. If there is a time change, the seq id is reset and the new time is used.
    Realtime,
}

/// The WrappedBowl is a thread-safe wrapper around the Bowl.
#[derive(Debug, Clone)]
pub struct WrappedBowl(Arc<Mutex<Bowl>>);

impl WrappedBowl {
    /// Creates a new WrappedBowl instance with the given node id, generation behavior mode and optional epoch.
    ///
    /// # Arguments
    ///
    /// * `node` - The node id for the WrappedBowl instance.
    /// * `mode` - The generation behavior mode for the WrappedBowl instance.
    /// * `epoch` - An optional epoch for the WrappedBowl instance.
    ///
    /// # Returns
    ///
    /// A new WrappedBowl instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::bowl::{GenerationBehavior, WrappedBowl};
    /// use std::time::SystemTime;
    ///
    /// let wrapped_bowl = WrappedBowl::of(1, GenerationBehavior::Normal, Some(SystemTime::now()));
    /// ```

    pub fn of(node: u8, mode: GenerationBehavior, epoch: Option<SystemTime>) -> Self {
        WrappedBowl(Arc::new(Mutex::new(Bowl::of(node, mode, epoch))))
    }

    /// Generates a new Oat value based on given parameters.
    ///
    /// # Returns
    ///
    /// A new Oat value.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::bowl::{GenerationBehavior, WrappedBowl};
    /// use std::time::SystemTime;
    ///
    /// let wrapped_bowl = WrappedBowl::of(1, GenerationBehavior::Normal, Some(SystemTime::now()));
    /// let oat = wrapped_bowl.generate();
    ///
    /// assert_eq!(oat.node(), 1);
    /// ```
    pub fn generate(&self) -> Oat {
        let node;
        let seq;
        let time;

        {
            let mut lock = self.0.lock().expect("Failed to get lock.");
            node = lock.node;
            seq = lock.new_seq();
            time = lock.last_timestamp;

            drop(lock)
        }

        Oat::of(node, seq, time)
    }
}

/// The Bowl is used for generating Oat values in a unified way.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Bowl {
    mode: GenerationBehavior,
    node: u8,
    epoch: Option<SystemTime>,
    current_seq: u16,    // max 12 bits (= 1,5 bytes)
    last_timestamp: u64, // max 44 bits (= 5,5 bytes)
}

impl Bowl {
    pub(crate) fn of(node: u8, mode: GenerationBehavior, epoch: Option<SystemTime>) -> Self {
        Bowl {
            mode,
            node,
            epoch,
            current_seq: 0,
            last_timestamp: get_time_millis(epoch),
        }
    }

    pub(crate) fn new_seq(&mut self) -> u16 {
        self.current_seq = (self.current_seq + 1) % 4096;
        let mut now_millis = get_time_millis(self.epoch);

        match self.mode {
            GenerationBehavior::Lazy => {
                if self.current_seq == 0 {
                    self.last_timestamp += 1;
                }
            }
            GenerationBehavior::Normal => {
                // Maintenance `last_time_millis` for every 4096 ids generated.
                if self.current_seq == 0 {
                    if now_millis == self.last_timestamp {
                        now_millis = biding_time_conditions(self.last_timestamp, self.epoch);
                    }

                    self.last_timestamp = now_millis;
                }
            }
            GenerationBehavior::Realtime => {
                // supplement code for 'clock is moving backwards situation'.

                // If the milliseconds of the current clock are equal to
                // the number of milliseconds of the most recently generated id,
                // then check if enough 4096 are generated,
                // if enough then busy wait until the next millisecond.
                if now_millis == self.last_timestamp {
                    if self.current_seq == 0 {
                        now_millis = biding_time_conditions(self.last_timestamp, self.epoch);
                        self.last_timestamp = now_millis;
                    }
                } else {
                    self.last_timestamp = now_millis;
                    self.current_seq = 0;
                }
            }
        }

        self.current_seq
    }
}

fn get_time_millis(epoch: Option<SystemTime>) -> u64 {
    SystemTime::now()
        .duration_since(epoch.unwrap_or(SystemTime::UNIX_EPOCH))
        .expect("Clock went backwards.")
        .as_millis() as u64
}

// Constantly refreshing the latest milliseconds by busy waiting.
fn biding_time_conditions(last_time_millis: u64, epoch: Option<SystemTime>) -> u64 {
    let mut latest_time_millis: u64;
    loop {
        latest_time_millis = get_time_millis(epoch);
        if latest_time_millis > last_time_millis {
            return latest_time_millis;
        }
        spin_loop();
    }
}
