run: src/cangjie.bin
	cargo r -r

src/cangjie.bin: cangjie5.dict.yaml
	python3 conv.py

cangjie5.dict.yaml:
	wget https://raw.githubusercontent.com/rime/rime-cangjie/master/cangjie5.dict.yaml

.PHONY: run
