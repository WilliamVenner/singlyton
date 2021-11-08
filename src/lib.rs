use std::{cell::UnsafeCell, mem::MaybeUninit};

#[cfg(test)]
mod tests;

mod cell;
use cell::*;

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
	pub fn get(&'static self) -> SinglytonRef<T> {
		self.0.get()
	}

	pub fn get_mut(&'static self) -> SinglytonRefMut<T> {
		self.0.get_mut()
	}

	pub fn as_ptr(&'static self) -> *const T {
		&*self.0.get() as *const T
	}

	pub fn as_mut_ptr(&'static self) -> *mut T {
		&mut *self.0.get_mut() as *mut T
	}

	pub fn replace(&'static self, val: T) {
		*self.0.get_mut() = val;
	}

	pub const fn new(val: T) -> Self {
		Self(SinglytonCell::new(val))
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
	#[cfg(debug_assertions)]
	fn uninit_check(&'static self) {
		if !unsafe { *self.initialized.get() } {
			panic!("This SingletonUninit has not been initialized yet");
		}
	}

	#[cfg(not(debug_assertions))]
	fn uninit_check(&'static self) {}

	pub fn get(&'static self) -> SinglytonRef<T> {
		self.uninit_check();
		map_ref(self.inner.get(), |maybe_uninit| unsafe {
			maybe_uninit.assume_init_ref()
		})
	}

	pub fn get_mut(&'static self) -> SinglytonRefMut<T> {
		self.uninit_check();
		map_ref_mut(self.inner.get_mut(), |maybe_uninit| unsafe {
			maybe_uninit.assume_init_mut()
		})
	}

	pub fn as_mut_ptr(&'static self) -> *mut T {
		self.uninit_check();
		self.inner.get_mut().as_mut_ptr()
	}

	pub fn as_ptr(&'static self) -> *const T {
		self.uninit_check();
		self.inner.get_mut().as_ptr()
	}

	pub fn replace(&'static self, val: T) {
		self.uninit_check();
		unsafe {
			let mut maybe_uninit = self.inner.get_mut();
			core::ptr::drop_in_place(maybe_uninit.as_mut_ptr());
			maybe_uninit.write(val);
		}
	}

	pub const fn uninit() -> Self {
		Self {
			inner: SinglytonCell::new(MaybeUninit::uninit()),
			initialized: UnsafeCell::new(false)
		}
	}

	pub const fn new(val: T) -> Self {
		Self {
			inner: SinglytonCell::new(MaybeUninit::new(val)),
			initialized: UnsafeCell::new(true)
		}
	}

	#[cfg(debug_assertions)]
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
	pub fn init(&'static self, val: T) {
		unsafe {
			self.inner.get_mut().write(val);
		}
	}
}
