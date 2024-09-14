run-rel: ../src/lib.rs font
	cargo r -r

run-deb: ../src/lib.rs font
	cargo r

font: FSung.zip
	unzip -n FSung.zip -d font -x FSung-F.ttf FSung-m.ttf FSung-X.ttf *.reg *.txt
	find font/ -name FSung-p.ttf | grep .
	find font/ -name FSung-1.ttf | grep .
	find font/ -name FSung-2.ttf | grep .
	find font/ -name FSung-3.ttf | grep .

FSung.zip:
	gdown 1m0-WYAXbEz3lxJrti25ZvWv6LkHjMp2X -O FSung.zip

../src/lib.rs: ../Makefile ../cangjie5.dict.yaml ../conv.py
	make -C .. src/lib.rs

.PHONY: run-rel run-deb
