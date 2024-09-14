run-rel: ../src/lib.rs
	cargo r -r -- -ea

run-deb: ../src/lib.rs
	cargo r -- -ea

../src/lib.rs: ../Makefile ../conv.py
	make -C .. src/lib.rs

.PHONY: run-rel run-deb
