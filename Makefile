all:
	cargo-watch -x run

ngrok:
	ngrok http http://localhost:8000 

load_commands:
	python loadCommands/main.py

test:
	curl -H 'Content-Type: application/json' -H 'X-Signature-Timestamp: 1742360234' -H 'X-Signature-Ed25519: 82c7f308d586ab1dd50af87e12ac11a17b0b6b0d9ece4a320e11db8f6aa3c493ef8bcfa824058f41338d3b8e4ecf5c7ec7d18b740c767d655f03ee0827a4db08' --data '{"app_permissions":"562949953601536","application_id":"1351410609454845953","authorizing_integration_owners":{},"entitlements":[],"id":"1351430620915437629","token":"aW50ZXJhY3Rpb246MTM1MTQzMDYyMDkxNTQzNzYyOTpveHFqamRDNGQxdkJYUndyaVplVUxoVnNFQ1dkY0xvR1JPcWZHVXBBWE1pY2JzYkdNUllKUHpaYnpNSWE4alF4U1RhcWloaHcxV3dXa3F5akFYeTlBOFAzaWVSMlVieGZZeXNJMU1RSDRuRDhidDBYNWNST1lyN0xKdEF6Rm5FTw","type":1,"user":{"avatar":"c6a249645d46209f337279cd2ca998c7","avatar_decoration_data":null,"bot":true,"clan":null,"collectibles":null,"discriminator":"0000","global_name":"Discord","id":"643945264868098049","primary_guild":null,"public_flags":1,"system":true,"username":"discord"},"version":1}' localhost:8000/interactions
