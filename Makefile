all:
	cd system && cargo build
	./system/target/debug/system
