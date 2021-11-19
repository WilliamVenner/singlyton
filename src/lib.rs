#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

#[cfg(test)]
mod tests;

mod cell;
use cell::*;

#[cfg(debug_assertions)]
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

/// A **thread-unsafe** global singleton.
///
/// Using this across threads is undefined behaviour.
///
/// # Panics
///
/// In debug builds, usage of this abstraction is checked for safety at runtime.
///
/// * Using this struct across threads will panic.
/// * Mixing mutabilty of borrows will panic (this is bypassed if you are using the pointer getters)
pub struct Singleton<T>(SinglytonCell<T>);
unsafe impl<T> Sync for Singleton<T> {}

impl<T> Singleton<T> {
	pub const fn new(val: T) -> Self {
		Self(SinglytonCell::new(val))
	}

	/// Acquires an **immutable reference** to the singleton.
	///
	/// In debug builds, this will panic if the singleton is accessed from a different thread or if a mutable reference is currently held.
	pub fn get(&'static self) -> SinglytonRef<T> {
		self.0.get()
	}

	/// Acquires a **mutable reference** to the singleton.
	///
	/// In debug builds, this will panic if the singleton is accessed from a different thread or an existing mutable or immutable reference is currently held.
	pub fn get_mut(&'static self) -> SinglytonRefMut<T> {
		self.0.get_mut()
	}

	/// Acquires an **immutable pointer** to the singleton.
	///
	/// In debug builds, this will panic if the singleton is accessed from a different thread or if a mutable reference is currently held.
	///
	/// This is unsafe because the returned pointer bypasses any future borrow checking.
	pub unsafe fn as_ptr(&'static self) -> *const T {
		&*self.0.get() as *const T
	}

	/// Acquires a **mutable pointer** to the singleton.
	///
	/// In debug builds, this will panic if the singleton is accessed from a different thread or an existing mutable or immutable reference is currently held.
	///
	/// This is unsafe because the returned pointer bypasses any future borrow checking.
	pub unsafe fn as_mut_ptr(&'static self) -> *mut T {
		&mut *self.0.get_mut() as *mut T
	}

	/// Replaces the value in the singleton with anew.
	///
	/// In debug builds, this will panic if the singleton is accessed from a different thread or an existing mutable or immutable reference is currently held.
	pub fn replace(&'static self, val: T) {
		*self.0.get_mut() = val;
	}
}

/// A **thread-unsafe** global singleton which is initially uninitialized memory.
///
/// Using this across threads is undefined behaviour.
///
/// # Panics
///
/// In debug builds, usage of this abstraction is checked for safety at runtime.
///
/// * Using this struct across threads will panic.
/// * Mixing mutabilty of borrows will panic (this is bypassed if you are using the pointer getters)
/// * Using this struct before initializing it will panic.
/// * Initializing the value more than once will panic. Use `replace`
pub struct SingletonUninit<T> {
	inner: SinglytonCell<MaybeUninit<T>>,

	#[cfg(debug_assertions)]
	initialized: UnsafeCell<bool>
}
unsafe impl<T> Sync for SingletonUninit<T> {}

impl<T> SingletonUninit<T> {
	pub const fn uninit() -> Self {
		Self {
			inner: SinglytonCell::new(MaybeUninit::uninit()),

			#[cfg(debug_assertions)]
			initialized: UnsafeCell::new(false)
		}
	}

	pub const fn new(val: T) -> Self {
		Self {
			inner: SinglytonCell::new(MaybeUninit::new(val)),

			#[cfg(debug_assertions)]
			initialized: UnsafeCell::new(true)
		}
	}

	#[cfg(debug_assertions)]
	fn uninit_check(&'static self) {
		if !unsafe { *self.initialized.get() } {
			panic!("This SingletonUninit has not been initialized yet");
		}
	}

	#[cfg(not(debug_assertions))]
	fn uninit_check(&'static self) {}

	/// Assumes the memory is **initialized** and acquires an **immutable reference** to the singleton.
	///
	/// In debug builds, this will panic if the memory is not initialized, the singleton is accessed from a different thread, or a mutable reference is currently held.
	pub fn get(&'static self) -> SinglytonRef<T> {
		self.uninit_check();
		map_ref(self.inner.get(), |maybe_uninit| unsafe {
			maybe_uninit.assume_init_ref()
		})
	}

	/// Acquires a **mutable reference** to the singleton.
	///
	/// In debug builds, this will panic if the memory is not initialized, the singleton is accessed from a different thread, or an existing mutable or immutable reference is currently held.
	pub fn get_mut(&'static self) -> SinglytonRefMut<T> {
		self.uninit_check();
		map_ref_mut(self.inner.get_mut(), |maybe_uninit| unsafe {
			maybe_uninit.assume_init_mut()
		})
	}

	/// Acquires an **immutable pointer** to the singleton.
	///
	/// In debug builds, this will panic if the memory is not initialized, the singleton is accessed from a different thread, or a mutable reference is currently held.
	///
	/// This is unsafe because the returned pointer bypasses any future borrow checking.
	pub fn as_ptr(&'static self) -> *const T {
		self.uninit_check();
		self.inner.get_mut().as_ptr()
	}

	/// Acquires a **mutable pointer** to the singleton.
	///
	/// In debug builds, this will panic if the memory is not initialized, the singleton is accessed from a different thread, or an existing mutable or immutable reference is currently held.
	///
	/// This is unsafe because the returned pointer bypasses any future borrow checking.
	pub fn as_mut_ptr(&'static self) -> *mut T {
		self.uninit_check();
		self.inner.get_mut().as_mut_ptr()
	}

	/// Replaces the value in the singleton with anew.
	///
	/// In debug builds, this will panic if the memory is not initialized, the singleton is accessed from a different thread, or an existing mutable or immutable reference is currently held.
	pub fn replace(&'static self, val: T) {
		self.uninit_check();
		unsafe {
			#[cfg(debug_assertions)]
			let mut maybe_uninit = self.inner.get_mut();

			#[cfg(not(debug_assertions))]
			let maybe_uninit = self.inner.get_mut();

			core::ptr::drop_in_place(maybe_uninit.as_mut_ptr());
			maybe_uninit.write(val);
		}
	}

	#[cfg(debug_assertions)]
	/// Initializes the memory in the singleton.
	///
	/// In debug builds, this will panic if the memory is **already initialized**, the singleton is accessed from a different thread, or an existing mutable or immutable reference is currently held.
	pub fn init(&'static self, val: T) {
		unsafe {
			let ref mut initialized = *self.initialized.get();
			if *initialized {
				panic!("This SingletonUninit has already been initialized");
			}

			self.inner.get_mut().write(val);

			*initialized = true;
		}
	}

	#[cfg(not(debug_assertions))]
	/// Initializes the memory in the singleton.
	///
	/// In debug builds, this will panic if the memory is **already initialized**, the singleton is accessed from a different thread, or an existing mutable or immutable reference is currently held.
	pub fn init(&'static self, val: T) {
		self.inner.get_mut().write(val);
	}
}
