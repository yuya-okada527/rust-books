run:
	cargo run -- input.txt -v
err:
	cargo run --bin err_panic
err_num:
	echo hoge > number.txt
	cargo run --bin err_panic
	rm number.txt
test:
	cargo test
ca:
	git add .
	git commit -m "commit all"
	git push origin head
