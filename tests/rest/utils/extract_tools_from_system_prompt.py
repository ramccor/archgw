import json


def extract_tools(system_prompt):
    l = system_prompt.rfind("<tools>")
    r = system_prompt.rfind("</tools>")

    if l != -1 and r != -1:
        tool_content = system_prompt[l + len("<tools>") : r]
        tools = [json.loads(tool) for tool in tool_content.split("\n") if tool]
        return tools
    else:
        raise ValueError("Invalid system prompt")


if __name__ == "__main__":
    system_prompt = 'You are a helpful assistant designed to assist with the user query by making one or more function calls if needed.\n\nYou are provided with function signatures within <tools></tools> XML tags:\n<tools>\n{"id": "get_new_releases", "type": "function", "function": {"name": "get_new_releases", "description": "Get a list of new album releases featured in Spotify (shown, for example, on a Spotify player\\u2019s \'Browse\' tab).", "parameters": {"type": "object", "properties": {"country": {"type": "str", "description": "The country where the album is released", "in_path": true}, "limit": {"type": "integer", "description": "The maximum number of results to return", "default": 5}}, "required": ["country"]}}}\n{"id": "search_for_item", "type": "function", "function": {"name": "search_for_item", "description": "Get information about albums, artists, playlists, tracks, shows, episodes, or audiobooks. You can search for an item by its name, creator, or topic.", "parameters": {"type": "object", "properties": {"q": {"type": "str", "description": "Your search query, which can include keywords related to the item name, its creator, or its topic."}, "type": {"type": "str", "description": "The type of the item to search for (e.g., album, artist, playlist, track, show, episode, audiobook).", "enum": ["album", "artist", "playlist", "track", "show", "episode", "audiobook"]}, "market": {"type": "str", "description": "A country code", "default": "US"}, "limit": {"type": "integer", "description": "The maximum number of results to return", "default": 5}}, "required": ["q", "type"]}}}\n</tools>\n\nYour task is to decide which functions are needed and collect missing parameters if necessary.\n\nBased on your analysis, provide your response in one of the following JSON formats:\n1. If no functions are needed:\n```\n{"response": "Your response text here"}\n```\n2. If functions are needed but some required parameters are missing:\n```\n{"required_functions": ["func_name1", "func_name2", ...], "clarification": "Text asking for missing parameters"}\n```\n3. If functions are needed and all required parameters are available:\n```\n{"tool_calls": [{"name": "func_name1", "arguments": {"argument1": "value1", "argument2": "value2"}},... (more tool calls as required)]}\n```'

    tools = extract_tools(system_prompt)
    print(json.dumps(tools, indent=4))
