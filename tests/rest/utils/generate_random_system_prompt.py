import json
import random

from datasets import load_dataset
from typing import Any, Dict, List


ARCH_FUNCTION_TOOL_PROMPT = (
    "You are a helpful assistant designed to assist with the user query by making one or more function calls if needed."
    "\n\nYou are provided with function signatures within <tools></tools> XML tags:\n<tools>{tool_text}\n</tools>"
    "\n\nYour task is to decide which functions are needed and collect missing parameters if necessary.\n\n"
)

ARCH_FUNCTION_FORMAT_PROMPT = """
Based on your analysis, provide your response in one of the following JSON formats:
1. If no functions are needed:
```
{"response": "Your response text here"}
```
2. If functions are needed but some required parameters are missing:
```
{"required_functions": ["func_name1", "func_name2", ...], "clarification": "Text asking for missing parameters"}
```
3. If functions are needed and all required parameters are available:
```
{"tool_calls": [{"name": "func_name1", "arguments": {"argument1": "value1", "argument2": "value2"}},... (more tool calls as required)]}
```
""".strip()


DATA_TYPE_MAP = {
    "string": "str",
    "integer": "int",
    "boolean": "bool",
    "number": "float",
    "array": "list",
    "Dict": "dict",
    "List": "list",
}


def process_data_types(param_type):
    extracted = {"type": [], "optional": False, "default": None}

    param_type = [t.strip() for t in param_type.split(",")]

    for t in param_type:
        if t.startswith("optional"):
            if t == "optional":
                extracted["optional"] = True
        elif t.startswith("default"):
            if "=" in t:
                extracted["default"] = t.split("=")[-1].strip()
            else:
                extracted["default"] = t.split(" ")[-1].strip()
        else:
            extracted["type"].append(t)

    extracted["type"] = ", ".join(extracted["type"])

    return extracted


def convert_tool(tool):
    converted = {
        "name": tool["name"],
        "description": tool["description"],
        "parameters": {"type": "object", "properties": {}, "required": []},
    }

    for param_name, param_value in tool["parameters"].items():
        extracted = process_data_types(param_value["type"])

        if extracted["type"] in DATA_TYPE_MAP:
            extracted["type"] = DATA_TYPE_MAP[extracted["type"]]

        parameter = {
            "type": extracted["type"],
            "description": param_value["description"],
        }

        if "default" in param_value:
            parameter["default"] = param_value["default"]
        elif extracted["default"] is not None:
            parameter["default"] = extracted["default"]

        if "default" in parameter:
            if parameter["default"] is None:
                parameter.pop("default")
            else:
                try:
                    if len(parameter["default"]) == 0:
                        parameter.pop("default")
                except Exception:
                    pass

        converted["parameters"]["properties"][param_name] = parameter

        if not extracted["optional"]:
            converted["parameters"]["required"].append(param_name)

    return converted


def build_system_prompt(tools: List[Dict[str, Any]]) -> str:
    tool_text = ""
    for tool in tools:
        tool_text += "\n" + json.dumps(tool)

    return (
        ARCH_FUNCTION_TOOL_PROMPT.format(tool_text=tool_text)
        + ARCH_FUNCTION_FORMAT_PROMPT
    )


if __name__ == "__main__":
    xlam = load_dataset("Salesforce/xlam-function-calling-60k")

    idx = random.sample(range(len(xlam["train"])), k=1)
    example = xlam["train"][idx]

    print("=" * 50 + " Tools " + "=" * 50)
    tools = [convert_tool(tool) for tool in json.loads(example["tools"][0])]
    print(json.dumps(tools, indent=4))

    print("\n" + "=" * 50 + " System Prompt " + "=" * 50)
    system_prompt = build_system_prompt(tools)

    # print(repr(system_prompt.encode("unicode_escape").decode()))
    print(json.dumps(system_prompt))
