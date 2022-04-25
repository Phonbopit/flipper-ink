test:
	cd $(name) && cargo +nightly test
build:
	cd $(name) && cargo +nightly contract build