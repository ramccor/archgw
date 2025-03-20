tools = [
    {
        "name": "account_balance",
        "description": "Get the account balance",
        "parameters": {
            "type": "object",
            "properties": {
                "account_id": {"type": "str", "description": "The account ID"}
            },
            "required": ["account_id"],
        },
    },
    {
        "name": "transfer_funds",
        "description": "Transfer funds between accounts",
        "parameters": {
            "type": "object",
            "properties": {
                "from_account": {
                    "type": "str",
                    "description": "The account to transfer from",
                },
                "to_account": {
                    "type": "str",
                    "description": "The account to transfer to",
                },
                "amount": {"type": "int", "description": "The amount to transfer"},
            },
            "required": ["from_account", "to_account", "amount"],
        },
    },
    {
        "name": "account_transactions",
        "description": "Get the account transactions",
        "parameters": {
            "type": "object",
            "properties": {
                "account_id": {"type": "str", "description": "The account ID"},
                "start_date": {
                    "type": "str",
                    "description": "The start date of the transactions",
                },
                "end_date": {
                    "type": "str",
                    "description": "The end date of the transactions",
                },
            },
            "required": ["account_id"],
        },
    },
]
