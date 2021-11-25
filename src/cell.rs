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

	#[repr(transparent)]
	pub(crate) struct SinglytonCell<T>(AtomicRefCell<T>);

	impl<T> SinglytonCell<T> {
		#[inline]
		pub(crate) const fn new(val: T) -> SinglytonCell<T> {
			SinglytonCell(AtomicRefCell::new(val))
		}

		/*
		#[inline]
		pub(crate) fn map<U: ?Sized, F>(&'static self, f: F) -> SinglytonRef<U>
		where
			F: FnOnce(&T) -> &U
		{
			map_ref(self.get(), f)
		}

		#[inline]
		pub(crate) fn map_mut<U: ?Sized, F>(&'static self, f: F) -> SinglytonRefMut<U>
		where
			F: FnOnce(&mut T) -> &mut U
		{
			map_ref_mut(self.get_mut(), f)
		}
		*/

		#[inline]
		pub(crate) fn get(&'static self) -> SinglytonRef<T> {
			self.0.borrow()
		}

		#[inline]
		pub(crate) fn get_mut(&'static self) -> SinglytonRefMut<T> {
			self.0.borrow_mut()
		}

		#[inline]
		pub(crate) unsafe fn get_unchecked(&'static self) -> &'static T {
			&*self.0.as_ptr()
		}

		#[inline]
		pub(crate) unsafe fn get_mut_unchecked(&'static self) -> &'static mut T {
			&mut *self.0.as_ptr()
		}
	}
}

#[cfg(not(debug_assertions))]
mod cell {
	use core::{ops::{Deref, DerefMut}, fmt::Debug, cell::UnsafeCell};

	#[repr(transparent)]
	pub struct SinglytonRef<'a, T: ?Sized>(&'a T);
	impl<'a, T: ?Sized> Deref for SinglytonRef<'a, T> {
		type Target = T;

		#[inline]
		fn deref(&self) -> &T {
			self.0
		}
	}
	impl<'a, T: ?Sized + Debug + 'a> Debug for SinglytonRef<'a, T> {
		fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
			self.0.fmt(f)
		}
	}

	#[repr(transparent)]
	pub struct SinglytonRefMut<'a, T: ?Sized>(&'a mut T);
	impl<'a, T: ?Sized> Deref for SinglytonRefMut<'a, T> {
		type Target = T;

		#[inline]
		fn deref(&self) -> &T {
			self.0
		}
	}
	impl<'a, T: ?Sized> DerefMut for SinglytonRefMut<'a, T> {
		#[inline]
		fn deref_mut(&mut self) -> &mut T {
			self.0
		}
	}
	impl<'a, T: ?Sized + Debug + 'a> Debug for SinglytonRefMut<'a, T> {
		fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
			self.0.fmt(f)
		}
	}

	#[inline]
	pub fn map_ref<'a, T: ?Sized, U: ?Sized, F>(reference: SinglytonRef<'a, T>, f: F) -> SinglytonRef<'a, U>
	where
		F: FnOnce(&T) -> &U
	{
		SinglytonRef(f(reference.0))
	}

	#[inline]
	pub fn map_ref_mut<'a, T: ?Sized, U: ?Sized, F>(reference: SinglytonRefMut<'a, T>, f: F) -> SinglytonRefMut<'a, U>
	where
		F: FnOnce(&mut T) -> &mut U
	{
		SinglytonRefMut(f(reference.0))
	}

	#[repr(transparent)]
	pub(crate) struct SinglytonCell<T>(UnsafeCell<T>);

	impl<T> SinglytonCell<T> {
		#[inline]
		pub(crate) const fn new(val: T) -> SinglytonCell<T> {
			SinglytonCell(UnsafeCell::new(val))
		}

		/*
		#[inline]
		pub(crate) fn map<U: ?Sized, F>(&'static self, f: F) -> SinglytonRef<U>
		where
			F: FnOnce(&T) -> &U
		{
			map_ref(self.get(), f)
		}

		#[inline]
		pub(crate) fn map_mut<U: ?Sized, F>(&'static self, f: F) -> SinglytonRefMut<U>
		where
			F: FnOnce(&mut T) -> &mut U
		{
			map_ref_mut(self.get_mut(), f)
		}
		*/

		#[inline]
		pub(crate) fn get(&self) -> SinglytonRef<'_, T> {
			SinglytonRef(unsafe { &*self.0.get() })
		}

		#[inline]
		pub(crate) fn get_mut(&self) -> SinglytonRefMut<'_, T> {
			SinglytonRefMut(unsafe { &mut *self.0.get() })
		}

		#[inline]
		pub(crate) unsafe fn get_unchecked(&self) -> SinglytonRef<'_, T> {
			self.get()
		}

		#[inline]
		pub(crate) unsafe fn get_mut_unchecked(&self) -> SinglytonRefMut<'_, T> {
			self.get_mut()
		}
	}
}

pub use cell::*;