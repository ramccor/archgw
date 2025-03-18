# POST http://localhost:8282/v1/chat/completions
# Content-Type: application/json

# {
#   "model": "Arch-Function",
#   "messages": [
#     {
#       "role": "system",
#       "content": "You are a helpful assistant designed to assist with the user query by making one or more function calls if needed.\n\nYou are provided with function signatures within <tools></tools> XML tags:\n<tools>\n{\"name\": \"convert_currency\", \"description\": \"Convert currency from one to another\", \"parameters\": {\"type\": \"object\", \"properties\": {\"from_currency\": {\"type\": \"string\", \"description\": \"The currency to convert from\", \"default\": \"USD\"}, \"to_currency\": {\"type\": \"string\", \"description\": \"The currency to convert to\"}}, \"required\": [\"to_currency\"]}}\n</tools>\n\nYour task is to decide which functions are needed and collect missing parameters if necessary.\n\nBased on your analysis, provide your response in one of the following JSON formats:\n1. If no functions are needed:\n```\n{\"response\": \"Your response text here\"}\n```\n2. If functions are needed but some required parameters are missing:\n```\n{\"required_functions\": [\"func_name1\", \"func_name2\", ...], \"clarification\": \"Text asking for missing parameters\"}\n```\n3. If functions are needed and all required parameters are available:\n```\n{\"tool_calls\": [{\"name\": \"func_name1\", \"arguments\": {\"argument1\": \"value1\", \"argument2\": \"value2\"}},... (more tool calls as required)]}\n```"
#     },
#     {
#       "role": "user",
#       "content": "I need help in converting currency"
#     }
#   ],
#   "temperature": 0.6,
#   "top_p": 1.0,
#   "top_k": 10
# }

# HTTP 200
# [Asserts]
# header "content-type" == "application/json"
# jsonpath "$.model" matches /^Arch-Function/
# jsonpath "$.usage" != null
# # {"required_functions": ["convert_currency"], "clarification": "From which currency to which currency?"}
# jsonpath "$.choices[0].message.content" matches /required_functions.*convert_currency.*clarification/
# jsonpath "$.choices[0].message.role" == "assistant"

# POST http://localhost:8282/v1/chat/completions
# Content-Type: application/json

# {
#   "model": "Arch-Function",
#   "messages": [
#     {
#       "role": "system",
#       "content": "You are a helpful assistant designed to assist with the user query by making one or more function calls if needed.\n\nYou are provided with function signatures within <tools></tools> XML tags:\n<tools>\n{\"name\": \"convert_currency\", \"description\": \"Convert currency from one to another\", \"parameters\": {\"type\": \"object\", \"properties\": {\"from_currency\": {\"type\": \"string\", \"description\": \"The currency to convert from\", \"default\": \"USD\"}, \"to_currency\": {\"type\": \"string\", \"description\": \"The currency to convert to\"}}, \"required\": [\"to_currency\"]}}\n</tools>\n\nYour task is to decide which functions are needed and collect missing parameters if necessary.\n\nBased on your analysis, provide your response in one of the following JSON formats:\n1. If no functions are needed:\n```\n{\"response\": \"Your response text here\"}\n```\n2. If functions are needed but some required parameters are missing:\n```\n{\"required_functions\": [\"func_name1\", \"func_name2\", ...], \"clarification\": \"Text asking for missing parameters\"}\n```\n3. If functions are needed and all required parameters are available:\n```\n{\"tool_calls\": [{\"name\": \"func_name1\", \"arguments\": {\"argument1\": \"value1\", \"argument2\": \"value2\"}},... (more tool calls as required)]}\n```"
#     },
#     {
#       "role": "user",
#       "content": "I need help in converting currency"
#     },
#     {
#       "role": "assistant",
#       "content": "From which currency to which currency?"
#     },
#     {
#       "role": "user",
#       "content": "USD to INR"
#     }
#   ],
#   "temperature": 0.6,
#   "top_p": 1.0,
#   "top_k": 10
# }

# HTTP 200
# [Asserts]
# header "content-type" == "application/json"
# jsonpath "$.model" matches /^Arch-Function/
# jsonpath "$.usage" != null
# # {"tool_calls": [{"name": "convert_currency", "arguments": {"from_currency": "USD", "to_currency": "INR"}}]}
# jsonpath "$.choices[0].message.content" matches /tool_calls.*convert_currency.*USD.*INR/
# jsonpath "$.choices[0].message.role" == "assistant"


POST http://localhost:8282/v1/chat/completions
Content-Type: application/json

{
  "model": "Arch-Function",
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant designed to assist with the user query by making one or more function calls if needed.\n\nYou are provided with function signatures within <tools></tools> XML tags:\n<tools>\n{\"name\": \"convert_currency\", \"description\": \"Convert currency from one to another\", \"parameters\": {\"type\": \"object\", \"properties\": {\"from_currency\": {\"type\": \"string\", \"description\": \"The currency to convert from\", \"default\": \"USD\"}, \"to_currency\": {\"type\": \"string\", \"description\": \"The currency to convert to\"}}, \"required\": [\"to_currency\"]}}\n</tools>\n\nYour task is to decide which functions are needed and collect missing parameters if necessary.\n\nBased on your analysis, provide your response in one of the following JSON formats:\n1. If no functions are needed:\n```\n{\"response\": \"Your response text here\"}\n```\n2. If functions are needed but some required parameters are missing:\n```\n{\"required_functions\": [\"func_name1\", \"func_name2\", ...], \"clarification\": \"Text asking for missing parameters\"}\n```\n3. If functions are needed and all required parameters are available:\n```\n{\"tool_calls\": [{\"name\": \"func_name1\", \"arguments\": {\"argument1\": \"value1\", \"argument2\": \"value2\"}},... (more tool calls as required)]}\n```"
    },
    {
      "role": "user",
      "content": "I need help in converting currency"
    },
    {
      "role": "assistant",
      "content": "From which currency to which currency?"
    },
    {
      "role": "user",
      "content": "USD to JPY"
    },
    {
      "role": "observation",
      "content": "<tool_response>\n{'name': 'convert_currency', 'result': '400 JPY'}\n</tool_response>"
    },
    {
      "role": "assistant",
      "content": "Here is the result: 400 JPY"
    },
    {
      "role": "user",
      "content": "what is the rate for gbp in usd?"
    }
  ],
  "temperature": 0.6,
  "top_p": 1.0,
  "top_k": 10
}

HTTP 200
[Asserts]
header "content-type" == "application/json"
jsonpath "$.model" matches /^Arch-Function/
jsonpath "$.usage" != null
# {"tool_calls": [{"name": "convert_currency", "arguments": {"from_currency": "USD", "to_currency": "JPY"}}]}
jsonpath "$.choices[0].message.content" matches /tool_calls.*convert_currency.*USD.*JPY/
jsonpath "$.choices[0].message.role" == "assistant"
