/// Convenience functions for interacting with RwLocks
pub use std::sync::{LazyLock, RwLock};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

/// Gets a copy of a value inside a RwLock and immediately unlocks
///
/// Requires <T: Copy> such as a bool or usize
pub fn read_rwlock<T: Copy>(rwlock: &RwLock<T>) -> T {
    *rwlock.read().unwrap()
}

/// Gets a clone of a value inside a RwLock and immediately unlocks
///
/// Can be used if <T> is not Copy, such as Vec<u32>
pub fn read_rwlock_clone<T: Clone>(rwlock: &RwLock<T>) -> T {
    rwlock.read().unwrap().clone()
}

/// Assigns a new value to a RwLock and immediately unlocks
pub fn assign_rwlock<T>(rwlock: &RwLock<T>, new_val: T) {
    *rwlock.write().unwrap() = new_val
}

/// Locks a RwLock for writing and returns the guard
///
/// Don't forget to drop the guard as soon as you're finished with it
pub fn lock_write_rwlock<T>(rwlock: &RwLock<T>) -> RwLockWriteGuard<T> {
    rwlock.write().unwrap()
}

/// Locks a RwLock for reading and returns the guard
///
/// Don't forget to drop the guard as soon as you're finished with it
pub fn lock_read_rwlock<T>(rwlock: &RwLock<T>) -> RwLockReadGuard<T> {
    rwlock.read().unwrap()
}
