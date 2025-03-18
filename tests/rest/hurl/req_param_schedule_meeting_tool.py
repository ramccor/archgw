tools = [
    {
        "name": "schedule_meeting",
        "description": "Schedule a meeting",
        "parameters": {
            "type": "object",
            "properties": {
                "subject": {
                    "type": "string",
                    "description": "The subject of the meeting",
                },
                "start_date": {
                    "type": "string",
                    "description": "The start date of the meeting",
                },
                "start_time": {
                    "type": "string",
                    "description": "The start time of the meeting",
                },
                "participants": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Email addresses of participants",
                },
                "duration": {
                    "type": "string",
                    "description": "The duration of the meeting",
                    "default": "1 hour",
                },
                "location": {
                    "type": "string",
                    "description": "The location of the meeting",
                },
            },
            "required": ["subject", "start_date", "start_time", "participants"],
        },
    }
]
