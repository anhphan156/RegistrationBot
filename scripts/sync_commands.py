import json
import os
import requests

def main():
    url = 'https://discord.com/api/v10/applications/' + os.environ.get('APP_ID') + '/commands'
    headers = {
        'Authorization': 'Bot ' + os.environ.get('DISCORD_TOKEN'),
        'Content-Type': 'application/json; charset=UTF-8',
        'User-Agent': 'Registration Bot'
    }

    commands = []

    for filename in os.listdir('./commands'):
        if filename.endswith('.json'):
            path = os.path.join('./commands/', filename)

            with open(path, 'r', encoding='utf=8') as f:
                command = json.load(f)
                commands.append(command)

    res = requests.put(url, json=commands, headers=headers)
    print(res.json())

if __name__=="__main__":
    main()
