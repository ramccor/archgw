import json
from typing import Any, Dict, List


ARCH_FUNCTION_TOOL_PROMPT = (
    "You are a helpful assistant designed to assist with the user query by making one or more function calls if needed."
    "\n\nYou are provided with function signatures within <tools></tools> XML tags:\n<tools>\n{tools}\n</tools>"
    "\n\nYour task is to decide which functions are needed and collect missing parameters if necessary."
)

ARCH_FUNCTION_FORMAT_PROMPT = (
    "\n\nBased on your analysis, provide your response in one of the following JSON formats:"
    '\n1. If no functions are needed:\n```json\n{"response": "Your response text here"}\n```'
    '\n2. If functions are needed but some required parameters are missing:\n```json\n{"required_functions": ["func_name1", "func_name2", ...], "clarification": "Text asking for missing parameters"}\n```'
    '\n3. If functions are needed and all required parameters are available:\n```json\n{"tool_calls": [{"name": "func_name1", "arguments": {"argument1": "value1", "argument2": "value2"}},... (more tool calls as required)]}\n```'
)


tools = [
    {
        "name": "get_weather",
        "description": "Retrieves current weather for the given location.",
        "parameters": {
            "type": "object",
            "properties": {
                "location": {
                    "type": "str",
                    "description": "City and State e.g. New York, NY",
                },
                "units": {
                    "type": "str",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "Units the temperature will be returned in.",
                },
            },
            "required": ["location", "units"],
        },
    },
    {
        "name": "get_stock_price",
        "description": "Get the current stock price",
        "parameters": {
            "type": "object",
            "properties": {
                "symbol": {"type": "str", "description": "The stock symbol"}
            },
            "required": ["symbol"],
        },
    },
]


def build_system_prompt(tools: List[Dict[str, Any]]) -> str:
    tool_text = ""
    for tool in tools:
        tool_text += "\n" + json.dumps(tool)

    return (
        ARCH_FUNCTION_TOOL_PROMPT.format(tool_text=tool_text)
        + ARCH_FUNCTION_FORMAT_PROMPT
    )


if __name__ == "__main__":
    system_prompt = build_system_prompt(tools)

    # print(repr(system_prompt.encode("unicode_escape").decode()))
    print(json.dumps(system_prompt))
