src/lib.rs: cangjie5.dict.yaml conv.py
	python3 conv.py > src/lib.rs

cangjie5.dict.yaml:
	curl https://raw.githubusercontent.com/rime/rime-cangjie/master/cangjie5.dict.yaml \
		| grep -e '^[^ -ÿ]' \
		| sed "s/\s\S*'\S*//" > cangjie5.dict.yaml
