tools = [
    {
        "name": "exchange_rate",
        "description": "Get the current exchange rate",
        "parameters": {
            "type": "object",
            "properties": {
                "from_currency": {
                    "type": "str",
                    "description": "The currency to convert from",
                    "default": "USD",
                },
                "to_currency": {
                    "type": "str",
                    "description": "The currency to convert to",
                },
            },
            "required": ["to_currency"],
        },
    },
]
