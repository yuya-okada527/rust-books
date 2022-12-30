run:
	cargo run --bin samplecli input.txt -v
err:
	cargo run --bin err_panic
err_num:
	echo hoge > number.txt
	cargo run --bin err_panic
	rm number.txt
anyhow:
	cargo run --bin err_anyhow
anyhow_num:
	echo hoge > number.txt
	cargo run --bin err_anyhow
	rm number.txt
anyhow_success:
	echo 32 > number.txt
	cargo run --bin err_anyhow
	rm number.txt
thiserror:
	cargo run --bin err_thiserror
thiserror_num:
	echo hoge > number.txt
	cargo run --bin err_thiserror
	rm number.txt
thiserror_success:
	echo 32 > number.txt
	cargo run --bin err_thiserror
	rm number.txt
test:
	cargo test
ca:
	git add .
	git commit -m "commit all"
	git push origin head
