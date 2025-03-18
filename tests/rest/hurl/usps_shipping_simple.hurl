POST http://localhost:8282/v1/chat/completions
Content-Type: application/json

{
  "model": "Arch-Function",
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant designed to assist with the user query by making one or more function calls if needed.\n\nYou are provided with function signatures within <tools></tools> XML tags:\n<tools>\n{\"name\": \"verify_address\", \"description\": \"Verify the address\", \"parameters\": {\"type\": \"object\", \"properties\": {\"address\": {\"type\": \"string\", \"description\": \"The address to verify\"}}, \"required\": [\"address\"]}}\n{\"name\": \"track_shipment\", \"description\": \"Track the shipment\", \"parameters\": {\"type\": \"object\", \"properties\": {\"tracking_number\": {\"type\": \"string\", \"description\": \"The tracking number\"}}, \"required\": [\"tracking_number\"]}}\n{\"name\": \"generate_label\", \"description\": \"Generate the shipping label\", \"parameters\": {\"type\": \"object\", \"properties\": {\"sender\": {\"type\": \"string\", \"description\": \"The address to ship from\"}, \"recipient\": {\"type\": \"string\", \"description\": \"The address to ship to\"}, \"weight\": {\"type\": \"number\", \"description\": \"The weight of the package\"}}, \"required\": [\"sender\", \"recipient\", \"weight\"]}}\n{\"name\": \"calculate_shipping_rate\", \"description\": \"Calculate the shipping rate\", \"parameters\": {\"type\": \"object\", \"properties\": {\"sender\": {\"type\": \"string\", \"description\": \"The address to ship from\"}, \"recipient\": {\"type\": \"string\", \"description\": \"The address to ship to\"}, \"weight\": {\"type\": \"number\", \"description\": \"The weight of the package\"}}, \"required\": [\"sender\", \"recipient\", \"weight\"]}}\n</tools>\n\nYour task is to decide which functions are needed and collect missing parameters if necessary.\n\nBased on your analysis, provide your response in one of the following JSON formats:\n1. If no functions are needed:\n```\n{\"response\": \"Your response text here\"}\n```\n2. If functions are needed but some required parameters are missing:\n```\n{\"required_functions\": [\"func_name1\", \"func_name2\", ...], \"clarification\": \"Text asking for missing parameters\"}\n```\n3. If functions are needed and all required parameters are available:\n```\n{\"tool_calls\": [{\"name\": \"func_name1\", \"arguments\": {\"argument1\": \"value1\", \"argument2\": \"value2\"}},... (more tool calls as required)]}\n```"
    },
    {
      "role": "user",
      "content": "What can you do?"
    }
  ],
  "temperature": 0.6,
  "top_p": 1.0,
  "top_k": 10
}

# resp
# it should be something like .. "I can help you with various tasks such as verifying addresses, tracking ...
# but more often I see {"tool_calls": [{"name": "verify_address", "arguments": {"address": "123 Main St"}}, {"name": "track_shipment", "arguments": {"tracking_number": "ABC123"}}, {"name": "generate_label", "arguments": {"sender": "123 Main St", "recipient": "456 Elm St", "weight": 10}}, {"name": "calculate_shipping_rate", "arguments": {"sender": "123 Main St", "recipient": "456 Elm St", "weight": 10}}]}

HTTP 200
[Asserts]
header "content-type" == "application/json"
jsonpath "$.model" matches /^Arch-Function/
jsonpath "$.usage" != null
# ignore param value check for now
jsonpath "$.choices[0].message.content" matches /help/
jsonpath "$.choices[0].message.role" == "assistant"
