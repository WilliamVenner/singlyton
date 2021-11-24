#[cfg(debug_assertions)]
mod cell {
	use atomic_refcell::{AtomicRefCell, AtomicRef, AtomicRefMut};

	pub type SinglytonRef<T> = AtomicRef<'static, T>;
	pub type SinglytonRefMut<T> = AtomicRefMut<'static, T>;

	#[inline]
	pub fn map_ref<'a, T: ?Sized, U: ?Sized, F>(reference: AtomicRef<'a, T>, f: F) -> AtomicRef<'a, U>
	where
		F: FnOnce(&T) -> &U
	{
		AtomicRef::map(reference, f)
	}

	#[inline]
	pub fn map_ref_mut<'a, T: ?Sized, U: ?Sized, F>(reference: AtomicRefMut<'a, T>, f: F) -> AtomicRefMut<'a, U>
	where
		F: FnOnce(&mut T) -> &mut U
	{
		AtomicRefMut::map(reference, f)
	}

	pub(crate) struct SinglytonCell<T>(AtomicRefCell<T>);

	impl<T> SinglytonCell<T> {
		pub(crate) const fn new(val: T) -> SinglytonCell<T> {
			SinglytonCell(AtomicRefCell::new(val))
		}

		pub(crate) fn get(&'static self) -> SinglytonRef<T> {
			self.0.borrow()
		}

		pub(crate) fn get_mut(&'static self) -> SinglytonRefMut<T> {
			self.0.borrow_mut()
		}

		pub(crate) unsafe fn get_unchecked(&'static self) -> &'static T {
			&*self.0.as_ptr()
		}

		pub(crate) unsafe fn get_mut_unchecked(&'static self) -> &'static mut T {
			&mut *self.0.as_ptr()
		}
	}
}

#[cfg(not(debug_assertions))]
mod cell {
	use core::cell::UnsafeCell;

	pub type SinglytonRef<T> = &'static T;
	pub type SinglytonRefMut<T> = &'static mut T;

	#[inline]
	pub fn map_ref<'a, T: ?Sized, U: ?Sized, F>(reference: &'a T, f: F) -> &'a U
	where
		F: FnOnce(&T) -> &U
	{
		f(reference)
	}

	#[inline]
	pub fn map_ref_mut<'a, T: ?Sized, U: ?Sized, F>(reference: &'a mut T, f: F) -> &'a mut U
	where
		F: FnOnce(&mut T) -> &mut U
	{
		f(reference)
	}

	pub(crate) struct SinglytonCell<T>(UnsafeCell<T>);

	impl<T> SinglytonCell<T> {
		pub(crate) const fn new(val: T) -> SinglytonCell<T> {
			SinglytonCell(UnsafeCell::new(val))
		}

		pub(crate) fn get(&self) -> &T {
			unsafe { &*self.0.get() }
		}

		pub(crate) fn get_mut(&self) -> &mut T {
			unsafe { &mut *self.0.get() }
		}

		pub(crate) unsafe fn get_unchecked(&'static self) -> &'static T {
			self.get()
		}

		pub(crate) unsafe fn get_mut_unchecked(&'static self) -> &'static mut T {
			self.get_mut()
		}
	}
}

pub use cell::*;