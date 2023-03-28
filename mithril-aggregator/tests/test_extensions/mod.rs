// Avoid clippy warnings generated by tests that doesn't use every tests_extensions (since each test
// is a different compilation target).
#![allow(dead_code)]

#[macro_use]
pub mod runtime_tester;
//mod dependency;
#[macro_use]
pub mod utilities;

//pub use dependency::initialize_dependencies;
pub use runtime_tester::RuntimeTester;
