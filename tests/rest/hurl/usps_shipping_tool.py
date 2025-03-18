tools = [
    {
        "name": "verify_address",
        "description": "Verify the address",
        "parameters": {
            "type": "object",
            "properties": {
                "address": {"type": "string", "description": "The address to verify"}
            },
            "required": ["address"],
        },
    },
    {
        "name": "track_shipment",
        "description": "Track the shipment",
        "parameters": {
            "type": "object",
            "properties": {
                "tracking_number": {
                    "type": "string",
                    "description": "The tracking number",
                }
            },
            "required": ["tracking_number"],
        },
    },
    {
        "name": "generate_label",
        "description": "Generate the shipping label",
        "parameters": {
            "type": "object",
            "properties": {
                "sender": {"type": "string", "description": "The address to ship from"},
                "recipient": {
                    "type": "string",
                    "description": "The address to ship to",
                },
                "weight": {
                    "type": "number",
                    "description": "The weight of the package",
                },
            },
            "required": ["sender", "recipient", "weight"],
        },
    },
    {
        "name": "calculate_shipping_rate",
        "description": "Calculate the shipping rate",
        "parameters": {
            "type": "object",
            "properties": {
                "sender": {"type": "string", "description": "The address to ship from"},
                "recipient": {
                    "type": "string",
                    "description": "The address to ship to",
                },
                "weight": {
                    "type": "number",
                    "description": "The weight of the package",
                },
            },
            "required": ["sender", "recipient", "weight"],
        },
    },
]
