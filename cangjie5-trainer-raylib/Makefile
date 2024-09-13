run-rel: ../src/lib.rs noto_sans_hk.ttf
	cargo r -r

run-deb: ../src/lib.rs noto_sans_hk.ttf
	cargo r

noto_sans_hk.ttf: noto_sans_hk_v.ttf
	fonttools varLib.mutator noto_sans_hk_v.ttf wght=400 wdth=90
	mv noto_sans_hk_v-instance.ttf noto_sans_hk.ttf

noto_sans_hk_v.ttf:
	curl "https://raw.githubusercontent.com/google/fonts/main/ofl/notosanshk/NotoSansHK%5bwght%5d.ttf" -o noto_sans_hk_v.ttf

../src/lib.rs: ../Makefile ../cangjie5.dict.yaml ../conv.py
	make -C .. src/lib.rs

.PHONY: run-rel run-deb
