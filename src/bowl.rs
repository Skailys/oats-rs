use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasherDefault};
use std::hint::spin_loop;

use std::sync::Mutex;
use std::time::{SystemTime};



use crate::oat::Oat;

// const EPOCH: u64 = 1577836800;
pub type BuildHasherFallback = BuildHasherDefault<DefaultHasher>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum GenerationBehavior {
    Lazy,
    Normal,
    Realtime
}

pub struct WrappedBowl(Mutex<Bowl>);

impl WrappedBowl {
    pub fn of(node: u8, mode: GenerationBehavior, epoch: Option<SystemTime>) -> Self {
        WrappedBowl(Mutex::new(Bowl::of(node, mode, epoch)))
    }

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

pub struct Bowl{
    mode: GenerationBehavior,
    node: u8,
    epoch: Option<SystemTime>,
    current_seq: u16, // max 12 bits (= 1,5 bytes)
    last_timestamp: u64, // max 44 bits (= 5,5 bytes)
}

impl Bowl {
    pub fn of(node: u8, mode: GenerationBehavior, epoch: Option<SystemTime>) -> Self {
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
            },
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
        .duration_since(epoch.unwrap_or(SystemTime::UNIX_EPOCH)).expect("Clock went backwards.")
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
