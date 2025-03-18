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
    },
    {
      "role": "assistant",
      "content": "I can help you with various tasks such as verifying addresses, tracking shipments, generating shipping labels, and calculating shipping rates. Would you like assistance with something specific?"
    },
    {
      "role": "user",
      "content": "I want to verify the address 123 Main St Redmond WA 98052"
    },
    {
      "role": "assistant",
      "content": "{\"tool_calls\": [{\"name\": \"verify_address\", \"arguments\": {\"address\": \"123 Main St Redmond WA 98052\"}}]}"
    },
    {
      "role": "user",
      "content": "<tool_response>\n{'name': 'verify_address', 'result': 'success'}\n</tool_response>"
    },
    {
      "role": "assistant",
      "content": "The address 123 Main St Redmond WA 98052 is valid."
    },
    {
      "role": "user",
      "content": "I want to track the shipment with tracking number 1234567890"
    },
    {
      "role": "assistant",
      "content": "{\"tool_calls\": [{\"name\": \"track_shipment\", \"arguments\": {\"tracking_number\": \"1234567890\"}}]}"
    },
    {
      "role": "user",
      "content": "<tool_response>\n{'name': 'track_shipment', 'result': 'shipped'}\n</tool_response>"
    },
    {
      "role": "assistant",
      "content": "The shipment with tracking number 1234567890 has been shipped."
    },
    {
      "role": "user",
      "content": "I want to generate a shipping label"
    },
    {
      "role": "assistant",
      "content": "Could you please provide the sender's address, the recipient's address, and the weight of the package?"
    },
    {
      "role": "user",
      "content": "Sure, sender address is 123 Main St Redmond WA 98052, recipient address is 456 Elm St Seattle WA 98101"
    },
    {
      "role": "assistant",
      "content": "Could you please provide me with the weight of the package?"
    },
    {
      "role": "user",
      "content": "The weight of the package is 5 lbs"
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
# ignore param value check for now
jsonpath "$.choices[0].message.content" matches /tool_calls.*generate_label/
jsonpath "$.choices[0].message.role" == "assistant"
