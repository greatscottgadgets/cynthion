# Depends on:
#
#	 cargo install svd2rust form

SVD := ../../cynthion/python/build/gensvd/moondancer.svd

svd2rust:
	rm -rf build src/generated.rs src/generated/
	mkdir build

	cp $(SVD) svd/moondancer.svd
	svd2rust -i svd/moondancer.svd -o build/ --target riscv --make_mod --ident-formats-theme legacy
	mv build/mod.rs src/generated.rs
	@# TODO uncomment once we no longer have to poly-fill 'target_has_atomic'
	@#		see: https://github.com/rust-lang/rust/issues/94039
	@# mv build/build.rs .
	mv build/device.x .

	form -i src/generated.rs -o src/generated/
	mv src/generated/lib.rs src/generated.rs
	cargo fmt -- --emit files

doc:
	cargo doc --no-deps --all-features --document-private-items --open

distclean:
	rm -rf build/ target/ src/generated.rs src/generated/ svd/moondancer.svd
