pkg: ../src/lib.rs src Cargo.toml font
	if [ "$(DEBUG)" = "1" ]; then \
		wasm-pack build --target web --dev; \
	else \
		wasm-pack build --target web; \
	fi

font: FSung.zip
	unzip -n FSung.zip -d font -x FSung-1.ttf FSung-F.ttf FSung-m.ttf FSung-X.ttf *.reg *.txt
	find font/ -name FSung-p.ttf | grep .
	find font/ -name FSung-2.ttf | grep .
	find font/ -name FSung-3.ttf | grep .

FSung.zip:
	gdown 1m0-WYAXbEz3lxJrti25ZvWv6LkHjMp2X -O FSung.zip

../src/lib.rs: ../Makefile ../conv.py
	make -C .. src/lib.rs

.PHONY: pkg
