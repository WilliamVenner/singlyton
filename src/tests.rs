use crate::*;

#[test]
fn test_singleton() {
	static SINGLETON: Singleton<&'static str> = Singleton::new("Hello");
	debug_assert_eq!(*SINGLETON.get(), "Hello");

	SINGLETON.replace("Test");
	debug_assert_eq!(*SINGLETON.get(), "Test");

	*SINGLETON.get_mut() = "Test 2";
	debug_assert_eq!(*SINGLETON.get(), "Test 2");
}

#[test]
fn test_singleton_uninit() {
	static SINGLETON: SingletonUninit<String> = SingletonUninit::uninit();

	SINGLETON.init("Hello".to_string());
	debug_assert_eq!(SINGLETON.get().as_str(), "Hello");

	SINGLETON.replace("Test".to_string());
	debug_assert_eq!(SINGLETON.get().as_str(), "Test");

	*SINGLETON.get_mut() = "Test 2".to_string();
	debug_assert_eq!(SINGLETON.get().as_str(), "Test 2");
}

#[test]
#[should_panic]
fn test_singleton_uninit_panic() {
	static SINGLETON: SingletonUninit<String> = SingletonUninit::uninit();
	SINGLETON.get();
}

#[test]
#[should_panic]
fn test_refcell() {
	static SINGLETON: Singleton<&'static str> = Singleton::new("Hello");
	let _my_ref = SINGLETON.get();
	let _my_mut_ref = SINGLETON.get_mut();
}

#[test]
#[should_panic]
fn test_thread_safety() {
	static SINGLETON: Singleton<&'static str> = Singleton::new("Hello");
	SINGLETON.get();

	std::thread::spawn(|| {
		SINGLETON.get();
	}).join().unwrap();
}