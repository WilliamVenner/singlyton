use std::{cell::{Ref, RefCell, RefMut, UnsafeCell}, mem::MaybeUninit, thread::{self, ThreadId}, lazy::OnceCell};

macro_rules! impl_thread_check {
	() => {
		fn thread_check(&'static self) {
			match unsafe { &mut *self.thread.get() } {
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
	};
}

/// A **thread-unsafe** global singleton.
///
/// Using this across threads is undefined behaviour.
///
/// # Panics
///
/// In debug builds, usage of this abstraction is checked for safety at runtime.
///
/// * Using this struct across threads will panic.
/// * Mixing mutabilty of borrows will panic.
pub struct Singleton<T: 'static> {
	inner: RefCell<T>,
	thread: UnsafeCell<Option<ThreadId>>,
}
unsafe impl<T: 'static> Sync for Singleton<T> {}

impl<T: 'static> Singleton<T> {
	impl_thread_check!();

	#[inline(always)]
	pub fn get(&'static self) -> Ref<'static, T> {
		self.thread_check();
		self.inner.borrow()
	}

	#[inline(always)]
	pub fn get_mut(&'static self) -> RefMut<'static, T> {
		self.thread_check();
		self.inner.borrow_mut()
	}

	#[inline(always)]
	pub fn as_ptr(&'static self) -> *const T {
		self.thread_check();
		(&*self.inner.borrow()) as *const T
	}

	#[inline(always)]
	pub fn as_mut_ptr(&'static self) -> *mut T {
		self.thread_check();
		(&mut *self.inner.borrow_mut()) as *mut T
	}

	pub fn replace(&'static self, val: T) {
		self.thread_check();
		*self.inner.borrow_mut() = val;
	}

	#[inline(always)]
	pub const fn new(val: T) -> Self {
		Self {
			inner: RefCell::new(val),
			thread: UnsafeCell::new(None)
		}
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
/// * Mixing mutabilty of borrows will panic.
/// * Using this struct before initializing it will panic.
/// * Initializing the value more than once will panic.
pub struct SingletonUninit<T: 'static> {
	inner: RefCell<MaybeUninit<T>>,
	thread: UnsafeCell<Option<ThreadId>>,
	initialized: UnsafeCell<bool>
}
unsafe impl<T: 'static> Sync for SingletonUninit<T> {}

impl<T: 'static> SingletonUninit<T> {
	impl_thread_check!();

	fn uninit_check(&'static self) {
		if !unsafe { *self.initialized.get() } {
			panic!("This SingletonUninit has not been initialized yet");
		}
	}

	#[inline(always)]
	pub fn get(&'static self) -> Ref<'static, T> {
		self.uninit_check();
		self.thread_check();
		Ref::map(self.inner.borrow(), |maybe_uninit| unsafe {
			maybe_uninit.assume_init_ref()
		})
	}

	#[inline(always)]
	pub fn get_mut(&'static self) -> RefMut<'static, T> {
		self.uninit_check();
		self.thread_check();
		RefMut::map(self.inner.borrow_mut(), |maybe_uninit| unsafe {
			maybe_uninit.assume_init_mut()
		})
	}

	#[inline(always)]
	pub fn as_mut_ptr(&'static self) -> *mut T {
		self.uninit_check();
		self.thread_check();
		self.inner.borrow_mut().as_mut_ptr()
	}

	#[inline(always)]
	pub fn as_ptr(&'static self) -> *const T {
		self.uninit_check();
		self.thread_check();
		self.inner.borrow().as_ptr()
	}

	pub fn replace(&'static self, val: T) {
		self.uninit_check();
		self.thread_check();
		unsafe {
			let mut maybe_uninit = self.inner.borrow_mut();
			core::ptr::drop_in_place(maybe_uninit.as_mut_ptr());
			maybe_uninit.write(val);
		}
	}

	#[inline(always)]
	pub const fn uninit() -> Self {
		Self {
			inner: RefCell::new(MaybeUninit::uninit()),
			thread: UnsafeCell::new(None),
			initialized: UnsafeCell::new(false)
		}
	}

	pub fn init(&'static self, val: T) {
		self.thread_check();
		unsafe {
			let ref mut initialized = *self.initialized.get();
			if *initialized {
				panic!("This SingletonUninit has already been initialized");
			}
			self.thread_check();
			self.inner.borrow_mut().write(val);
			*initialized = true;
		}
	}

	pub fn init_or_replace(&'static self, val: T) {
		self.thread_check();
		unsafe {
			let ref mut initialized = *self.initialized.get();
			if *initialized {
				let mut maybe_uninit = self.inner.borrow_mut();
				core::ptr::drop_in_place(maybe_uninit.as_mut_ptr());
				maybe_uninit.write(val);
			} else {
				self.inner.borrow_mut().write(val);
				*initialized = true;
			}
		}
	}
}

/// A **thread-unsafe** global singleton which is initialized lazily.
///
/// Using this across threads is undefined behaviour.
///
/// # Panics
///
/// In debug builds, usage of this abstraction is checked for safety at runtime.
///
/// * Using this struct across threads will panic.
/// * Mixing mutabilty of borrows will panic.
pub struct SingletonLazy<T: 'static, F: FnOnce() -> T + 'static> {
	inner: OnceCell<RefCell<T>>,
	thread: UnsafeCell<Option<ThreadId>>,
	initializer: UnsafeCell<Option<F>>
}
unsafe impl<T: 'static, F: FnOnce() -> T> Sync for SingletonLazy<T, F> {}

impl<T: 'static, F: FnOnce() -> T + 'static> SingletonLazy<T, F> {
	impl_thread_check!();

	#[inline(always)]
	fn get_lazy(&'static self) -> &RefCell<T> {
		self.inner.get_or_init(|| RefCell::new((unsafe { &mut *self.initializer.get() }.take().unwrap())()))
	}

	#[inline(always)]
	pub fn get(&'static self) -> Ref<'static, T> {
		self.thread_check();
		self.get_lazy().borrow()
	}

	#[inline(always)]
	pub fn get_mut(&'static self) -> RefMut<'static, T> {
		self.thread_check();
		self.get_lazy().borrow_mut()
	}

	#[inline(always)]
	pub fn as_ptr(&'static self) -> *const T {
		self.thread_check();
		(&*self.get_lazy().borrow()) as *const T
	}

	#[inline(always)]
	pub fn as_mut_ptr(&'static self) -> *mut T {
		self.thread_check();
		(&mut *self.get_lazy().borrow_mut()) as *mut T
	}

	pub fn replace(&'static self, val: T) {
		self.thread_check();
		*self.get_lazy().borrow_mut() = val;
	}

	#[inline(always)]
	pub const fn new(initializer: F) -> Self {
		Self {
			inner: OnceCell::new(),
			thread: UnsafeCell::new(None),
			initializer: UnsafeCell::new(Some(initializer))
		}
	}
}