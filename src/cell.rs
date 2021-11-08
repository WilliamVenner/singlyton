#[cfg(any(debug_assertions, test))]
mod cell {
	use std::{cell::{Ref, RefMut, RefCell, UnsafeCell}, thread::{self, ThreadId}};

	pub(crate) type SinglytonRef<T> = Ref<'static, T>;
	pub(crate) type SinglytonRefMut<T> = RefMut<'static, T>;

	pub(crate) fn map_ref<T: ?Sized, U: ?Sized, F>(reference: Ref<'static, T>, f: F) -> Ref<'static, U>
	where
		F: FnOnce(&T) -> &U
	{
		Ref::map(reference, f)
	}

	pub(crate) fn map_ref_mut<T: ?Sized, U: ?Sized, F>(reference: RefMut<'static, T>, f: F) -> RefMut<'static, U>
	where
		F: FnOnce(&mut T) -> &mut U
	{
		RefMut::map(reference, f)
	}

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

		pub(crate) fn get(&self) -> Ref<'_, T> {
			assert_single_threaded(&self.thread);
			self.cell.borrow()
		}

		pub(crate) fn get_mut(&self) -> RefMut<'_, T> {
			assert_single_threaded(&self.thread);
			self.cell.borrow_mut()
		}
	}
}

#[cfg(not(any(debug_assertions, test)))]
mod cell {
	use std::cell::UnsafeCell;

	pub type SinglytonRef<T> = &'static T;
	pub type SinglytonRefMut<T> = &'static mut T;

	pub(crate) fn map_ref<T: ?Sized, U: ?Sized, F>(reference: &'static T, f: F) -> &'static U
	where
		F: FnOnce(&T) -> &U
	{
		f(reference)
	}

	pub(crate) fn map_ref_mut<T: ?Sized, U: ?Sized, F>(reference: &'static mut T, f: F) -> &'static mut U
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

pub(crate) use cell::*;