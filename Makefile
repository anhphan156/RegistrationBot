run:
	cargo-watch -x run

ngrok:
	ngrok http http://localhost:8000 

sync_commands:
	python script/sync_commands.py
