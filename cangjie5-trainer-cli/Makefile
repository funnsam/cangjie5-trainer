run-rel: ../src/lib.rs
	cargo r -r -- -ea

run-deb: ../src/lib.rs
	cargo r -- -ea

../src/lib.rs: ../Makefile ../read.py ../conv_rs.py
	make -C .. src/lib.rs

.PHONY: run-rel run-deb
