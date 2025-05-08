import json
import yaml

system_prompt = """
You are an advanced Routing Assistant designed to select the optimal route based on user requests.
Your task is to analyze conversations and match them to the most appropriate predefined route.
Review the available routes config:

# ROUTES CONFIG START
{routes}
# ROUTES CONFIG END

Examine the following conversation between a user and an assistant:

# CONVERSATION START
{conversation}
# CONVERSATION END

Your goal is to identify the most appropriate route that matches the user's LATEST intent. Follow these steps:

1. Carefully read and analyze the provided conversation, focusing on the user's latest request and the conversation scenario.
2. Check if the user's request and scenario matches any of the routes in the routing configuration (focus on the description).
3. Find the route that best matches.
4. Use context clues from the entire conversation to determine the best fit.
5. Return the best match possible. You only response the name of the route that best matches the user's request, use the exact name in the routes config.
6. If no route relatively close to matches the user's latest intent or user last message is thank you or greeting, return an empty route ''.
"""

output_format = """
# OUTPUT FORMAT
Your final output must follow this JSON format:
{
  "route": "route_name" # The matched route name, or empty string '' if no match
}

Based on your analysis, provide only the JSON object as your final output with no additional text, explanations, or whitespace.
"""


with open("arch_config.yaml", "r") as file:
    data = yaml.safe_load(file)

llm_provider_routes = ""

for llm_provider in data.get("llm_providers", []):
    llm_provider_routes += f"- name: {llm_provider.get('name')}()\n"
    llm_provider_routes += f"  description: {json.dumps(llm_provider.get('usage'))}\n"


conversation = """
user: Hello
assistant: Hi! How can I assist you today?
user: I want to know how far is sun from earth.
"""

system_prompt_formatted = system_prompt.format(
    routes=llm_provider_routes, conversation=conversation
)

system_prompt_2 = f"{system_prompt_formatted}\n{output_format}"
print(system_prompt_2)
print(json.dumps(system_prompt_2, indent=2))
