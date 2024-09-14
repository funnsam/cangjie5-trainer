run-rel: ../src/lib.rs
	cargo r -r

run-deb: ../src/lib.rs
	cargo r

../src/lib.rs: ../Makefile ../conv.py
	make -C .. src/lib.rs

.PHONY: run-rel run-deb
