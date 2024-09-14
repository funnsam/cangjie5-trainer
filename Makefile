src/lib.rs: cangjie5.dict.txt conv.py
	python3 conv.py

cangjie5.dict.txt:
	curl https://raw.githubusercontent.com/Jackchows/Cangjie5/master/Cangjie5.txt \
		| grep -P "^.\t" \
		| sed "s/\s\S*'\S*//" > cangjie5.dict.txt
