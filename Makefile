all:
	cargo-watch -x run

ngrok:
	ngrok http http://localhost:8000 

load_commands:
	python loadCommands/main.py

test:
	curl -H 'Content-Type: application/json' --data '{"app_permissions":"562949953601536","application_id":"1351410609454845953","authorizing_integration_owners":{},"entitlements":[],"id":"1351430620915437629","token":"aW50ZXJhY3Rpb246MTM1MTQzMDYyMDkxNTQzNzYyOTpveHFqamRDNGQxdkJYUndyaVplVUxoVnNFQ1dkY0xvR1JPcWZHVXBBWE1pY2JzYkdNUllKUHpaYnpNSWE4alF4U1RhcWloaHcxV3dXa3F5akFYeTlBOFAzaWVSMlVieGZZeXNJMU1RSDRuRDhidDBYNWNST1lyN0xKdEF6Rm5FTw","type":1,"user":{"avatar":"c6a249645d46209f337279cd2ca998c7","avatar_decoration_data":null,"bot":true,"clan":null,"collectibles":null,"discriminator":"0000","global_name":"Discord","id":"643945264868098049","primary_guild":null,"public_flags":1,"system":true,"username":"discord"},"version":1}' localhost:8000/interactions
