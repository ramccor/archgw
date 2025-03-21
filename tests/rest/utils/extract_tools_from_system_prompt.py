import json


def extract_tools(system_prompt):
    l = system_prompt.rfind("<tools>")
    r = system_prompt.rfind("</tools>")

    if l != -1 and r != -1:
        tool_content = system_prompt[l + len("<tools>") : r]
        print(tool_content.split("\n"))
        tools = [json.loads(tool) for tool in tool_content.split("\n") if tool]
        return tools
    else:
        raise ValueError("Invalid system prompt")


if __name__ == "__main__":
    system_prompt = 'You are a helpful assistant designed to assist with the user query by making one or more function calls if needed.\n\nYou are provided with function signatures within <tools></tools> XML tags:\n<tools>\n{"name": "verify_address", "description": "Verify the address", "parameters": {"type": "object", "property_address": {"type": "str", "description": "Complete address of the property", "format": "Street address, City, State Country"}, "required": ["property_address"]}}\n{"name": "track_shipment", "description": "Track the shipment", "parameters": {"type": "object", "properties": {"tracking_number": {"type": "str", "description": "The tracking number"}}, "required": ["tracking_number"]}}\n{"name": "generate_label", "description": "Generate the shipping label", "parameters": {"type": "object", "properties": {"sender": {"type": "str", "description": "The address to ship from"}, "recipient": {"type": "str", "description": "The address to ship to"}, "weight": {"type": "int", "description": "The weight of the package"}}, "required": ["sender", "recipient", "weight"]}}\n{"name": "calculate_shipping_rate", "description": "Calculate the shipping rate", "parameters": {"type": "object", "properties": {"sender": {"type": "str", "description": "The address to ship from"}, "recipient": {"type": "str", "description": "The address to ship to"}, "weight": {"type": "int", "description": "The weight of the package"}}, "required": ["sender", "recipient", "weight"]}}\n</tools>\n\nYour task is to decide which functions are needed and collect missing parameters if necessary.\n\nBased on your analysis, provide your response in one of the following JSON formats:\n1. If no functions are needed:\n```\n{"response": "Your response text here"}\n```\n2. If functions are needed but some required parameters are missing:\n```\n{"required_functions": ["func_name1", "func_name2", ...], "clarification": "Text asking for missing parameters"}\n```\n3. If functions are needed and all required parameters are available:\n```\n{"tool_calls": [{"name": "func_name1", "arguments": {"argument1": "value1", "argument2": "value2"}},... (more tool calls as required)]}\n```'

    tools = extract_tools(system_prompt)
    print(json.dumps(tools, indent=4))
