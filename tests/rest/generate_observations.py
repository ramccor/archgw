# Use the format `{"name": "function_name", "result": "function_result"}` for each tool call
tool_call_results = [
    {"name": "get_weather", "result": "37.1 f"}
    # {"name": "get_stock_price", "result": "247.66 USD"}
    # Add more results if needed
]


def build_observations(tool_call_results):
    observations = "\n".join([repr(x) for x in tool_call_results])
    observations = f"<tool_response>\n{observations}\n</tool_response>"
    return observations


if __name__ == "__main__":
    observations = build_observations(tool_call_results)
    print(repr(observations))
