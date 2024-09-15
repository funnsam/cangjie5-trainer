build: font script.min.js

font: FSung.zip
	unzip -n FSung.zip -d font -x FSung-1.ttf FSung-F.ttf FSung-m.ttf FSung-X.ttf *.reg *.txt
	find font/ -name FSung-p.ttf | grep .
	find font/ -name FSung-2.ttf | grep .
	find font/ -name FSung-3.ttf | grep .

FSung.zip:
	gdown 1m0-WYAXbEz3lxJrti25ZvWv6LkHjMp2X -O FSung.zip

script.min.js: script.js
	if ! command -v uglifyjs &> /dev/null; then \
		cp script.js script.min.js; \
	else \
		uglifyjs script.js -c -m -o script.min.js; \
	fi

script.js: _script.js
	cd ..; python3 conv_js.py $(shell pwd)/script.js
	cat _script.js >> script.js

.PHONY: build
