src/lib.rs: cangjie5.dict.txt conv.py Unihan_Variants.txt
	python3 conv.py

cangjie5.dict.txt:
	curl https://raw.githubusercontent.com/Jackchows/Cangjie5/master/Cangjie5.txt \
		| grep -P "^.\t" \
		| sed "s/\s\S*'\S*//" > cangjie5.dict.txt

Unihan_Variants.txt: Unihan.zip
	unzip -p Unihan.zip Unihan_Variants.txt \
		| grep -P "^[^\#][^\t]+\tkTraditionalVariant" > Unihan_Variants.txt

Unihan.zip:
	curl https://www.unicode.org/Public/UCD/latest/ucd/Unihan.zip -o Unihan.zip
