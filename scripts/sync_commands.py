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
    commands =  [
        {
            'name': 'create-event',
            'description': 'Basic command',
            'type': 1,
            'integration_types': [0, 1],
            'contexts': [0, 1, 2],
            'options': [
                {
                    'type': 3,
                    'name': 'time',
                    'description': 'UTC time for event',
                    'required': True
                },
                {
                    'type': 3,
                    'name': 'template',
                    'description': 'Role template for event',
                    'choices': [
                        {'name':'template3', 'value': 'https://pastebin.com/raw/1uMpGQNn'},
                        {'name':'template5', 'value': 'https://pastebin.com/raw/qcTsm6AC'}

                    ],
                    'required': True
                }
            ]
        }
    ];

    res = requests.put(url, json=commands, headers=headers)
    print(res.json())



if __name__=="__main__":
    main()
