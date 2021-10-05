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
