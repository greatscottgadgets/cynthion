check:
	cargo check --release --target riscv32imac-unknown-none-elf

fmt:
	cargo fmt -- --emit files

build:
	RUSTFLAGS="-C link-arg=-Tmemory.x -C link-arg=-Tlink.x" cargo build --release --target riscv32imac-unknown-none-elf


ALL_GROUPS=-Wclippy::correctness \
		   -Wclippy::suspicious \
		   -Wclippy::complexity \
		   -Wclippy::perf \
		   -Wclippy::style \
		   -Wclippy::pedantic \
		   -Wclippy::restriction \
		   -Wclippy::cargo

CLIPPY_GROUPS=-Wclippy::correctness \
			  -Wclippy::suspicious \
			  -Wclippy::complexity \
			  -Wclippy::perf \
			  -Wclippy::style \
			  -Wclippy::pedantic \
			  -Wclippy::cargo

CLIPPY_DISABLE=-Aclippy::enum_glob_use \
			   -Aclippy::module_name_repetitions \
			   -Aclippy::used_underscore_binding \
			   -Aclippy::match_same_arms \
			   -Aclippy::match_wild_err_arm \
			   -Aclippy::missing_errors_doc \
			   -Aclippy::no_effect_underscore_binding \
			   -Aclippy::unnecessary_wraps \
			   -Aclippy::single_match_else \
			   -Aclippy::cast_possible_truncation \
			   -Aclippy::inline_always \
			   -Aclippy::multiple_crate_versions \
			   -Aclippy::new-without-default

clippy:
	cargo clippy --target riscv32imac-unknown-none-elf -- -Dwarnings $(CLIPPY_GROUPS) $(CLIPPY_DISABLE)

doc:
	cargo doc --no-deps --document-private-items

doc-open:
	cargo doc --no-deps --document-private-items --open

doc-coverage:
	RUSTDOCFLAGS="-Z unstable-options --show-coverage" cargo +nightly doc --no-deps --document-private-items

test:
	cargo test
