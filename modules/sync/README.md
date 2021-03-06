# sync

- Module `std::sync`, 1.0.0
- Useful synchronization primitives.
- This module contains useful safe and unsafe synchronization primitives.
- Most of the primitives in this module do not provide any sort of locking and/or blocking at all, but rather provide the necessary tools to build other types of concurrent primitives.


## Modules
- `atomic` Atomic types
- `mpsc` Multi-producer, single-consumer FIFO queue communication primitives.

## Structs
- `Arc` a thread-safe reference-counting pointer. 'Arc' stands for Atomically Reference Counted.
- `Barrier` a barrier enables multiple threads to synchronize the beginning of some computation.
- `BarrierWaitResult` a BarrierWaitResult is returned by wait when all threads in the Barrier have rendezvoused.
- `Condvar` a Condition Variable
- `Mutex` a mutual exclusion primitive useful for protecting shared data
- `MutexGuard` an RAII implementation of a "scoped lock" of a mutex. When this structure is dropped (falls out of scope), the lock will be unlocked.
- `Once` a synchronization primitive which can be used to run a one-time global initialization. Useful for one-time initialization for FFI or related functionality. This type can only be constructed with the `ONCE_INIT` value.
- `PoisonError` type of error which can be returned whenever a lock is acquired.
- `RwLock` A reader-writer lock
- `RwLockReadGuard` RAII structure used to release the shared read access of a lock when dropped.
- `RwLockWriteGuard` RAII structure used to release the exclusive write access of a lock when dropped.
- `WaitTimeoutResult` A type indicating whether a timed wait on a condition variable returned due to a time out or not.
- `Weak` version of Arc that holds a non-owning reference to the managed value. The value is accessed by calling upgrade on the Weak pointer, which returns an Option<Arc<T>>.
- `OnceState` [LAB] State yielded to call_once_force's closure parameter. The state can be used to query the poison status of the Once.


## Enums
- `TryLockError` An enumeration of possible errors associated with a TryLockResult which can occur while trying to acquire a lock, from the try_lock method on a `Mutex` or the try_read and try_write methods on an `RwLock`.


## Constants
- `ONCE_INIT` Initialization value for static Once values.

## Type Definitions
- `LockResult` A type alias for the result of a lock method which can be poisoned.
- `TryLockResult` A type alias for the result of a non-blocking locking method.
