run:
	cargo-watch -x run

ngrok:
	ngrok http http://localhost:6969 

sync_commands:
	python scripts/sync_commands.py

deploy:
	scp ./result/bin/registration-bot bot@10.0.0.20:~/bin
