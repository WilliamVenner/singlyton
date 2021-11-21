#[cfg(any(debug_assertions, test))]
mod cell {
	use core::cell::{UnsafeCell, RefCell, Ref, RefMut};

	#[cfg(feature = "std")]
	use std::thread::{self, ThreadId};

	pub type SinglytonRef<T> = Ref<'static, T>;
	pub type SinglytonRefMut<T> = RefMut<'static, T>;

	#[inline]
	pub fn map_ref<'a, T: ?Sized, U: ?Sized, F>(reference: Ref<'a, T>, f: F) -> Ref<'a, U>
	where
		F: FnOnce(&T) -> &U
	{
		Ref::map(reference, f)
	}

	#[inline]
	pub fn map_ref_mut<'a, T: ?Sized, U: ?Sized, F>(reference: RefMut<'a, T>, f: F) -> RefMut<'a, U>
	where
		F: FnOnce(&mut T) -> &mut U
	{
		RefMut::map(reference, f)
	}

	#[cfg(feature = "std")]
	fn assert_single_threaded(thread_id: &UnsafeCell<Option<ThreadId>>) {
		match unsafe { &mut *thread_id.get() } {
			Some(thread_id) => {
				let this_thread_id = thread::current().id();
				if *thread_id != this_thread_id {
					panic!(concat!(stringify!($singleton), " was constructed in thread {:?}, but accessed in thread {:?}"), thread_id, this_thread_id);
				}
			},
			thread_id @ None => {
				*thread_id = Some(thread::current().id());
			}
		}
	}

	pub(crate) struct SinglytonCell<T> {
		cell: RefCell<T>,
		thread: UnsafeCell<Option<ThreadId>>
	}

	impl<T> SinglytonCell<T> {
		pub(crate) const fn new(val: T) -> SinglytonCell<T> {
			SinglytonCell {
				cell: RefCell::new(val),
				thread: UnsafeCell::new(None)
			}
		}

		pub(crate) fn get(&'static self) -> SinglytonRef<T> {
			#[cfg(feature = "std")]
			assert_single_threaded(&self.thread);
			self.cell.borrow()
		}

		pub(crate) fn get_mut(&'static self) -> SinglytonRefMut<T> {
			#[cfg(feature = "std")]
			assert_single_threaded(&self.thread);
			self.cell.borrow_mut()
		}
	}
}

#[cfg(not(any(debug_assertions, test)))]
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
	}
}

pub use cell::*;