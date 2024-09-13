src/lib.rs: cangjie5.dict.yaml conv.py
	python3 conv.py

cangjie5.dict.yaml:
	curl https://raw.githubusercontent.com/rime/rime-cangjie/master/cangjie5.dict.yaml \
		| grep -e "^[^ -\~]" \
		| sed "s/\s\S*'\S*//" > cangjie5.dict.yaml
