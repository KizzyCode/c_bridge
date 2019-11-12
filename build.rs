fn main() {
	#[cfg(feature = "test_build")] {
		cc::Build::new()
			.file("src/tests/test.c")
			.include("interfaces/c")
			.warnings_into_errors(true)
			.compile("test");
	}
}