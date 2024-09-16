run-rel: ../src/lib.rs font
	cargo r -r

run-deb: ../src/lib.rs font
	cargo r

font: FSung.zip
	unzip -n FSung.zip -d font -x FSung-1.ttf FSung-F.ttf FSung-m.ttf FSung-X.ttf *.reg *.txt
	find font/ -name FSung-p.ttf | grep .
	find font/ -name FSung-2.ttf | grep .
	find font/ -name FSung-3.ttf | grep .
	curl "https://raw.githubusercontent.com/adobe-fonts/source-code-pro/release/TTF/SourceCodePro-Regular.ttf" -o font/SourceCodePro.ttf

FSung.zip:
	gdown 1m0-WYAXbEz3lxJrti25ZvWv6LkHjMp2X -O FSung.zip

../src/lib.rs: ../Makefile ../read.py ../conv_rs.py
	make -C .. src/lib.rs

.PHONY: run-rel run-deb
