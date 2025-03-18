POST http://localhost:8282/v1/chat/completions
Content-Type: application/json

{
  "model": "Arch-Function",
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant designed to assist with the user query by making one or more function calls if needed.\n\nYou are provided with function signatures within <tools></tools> XML tags:\n<tools>\n{\"name\": \"account_balance\", \"description\": \"Get the account balance\", \"parameters\": {\"type\": \"object\", \"properties\": {\"account_id\": {\"type\": \"str\", \"description\": \"The account ID\"}}, \"required\": [\"account_id\"]}}\n{\"name\": \"transfer_funds\", \"description\": \"Transfer funds between accounts\", \"parameters\": {\"type\": \"object\", \"properties\": {\"from_account\": {\"type\": \"str\", \"description\": \"The account to transfer from\"}, \"to_account\": {\"type\": \"str\", \"description\": \"The account to transfer to\"}, \"amount\": {\"type\": \"int\", \"description\": \"The amount to transfer\"}}, \"required\": [\"from_account\", \"to_account\", \"amount\"]}}\n{\"name\": \"account_transactions\", \"description\": \"Get the account transactions\", \"parameters\": {\"type\": \"object\", \"properties\": {\"account_id\": {\"type\": \"str\", \"description\": \"The account ID\"}, \"start_date\": {\"type\": \"str\", \"description\": \"The start date of the transactions\"}, \"end_date\": {\"type\": \"str\", \"description\": \"The end date of the transactions\"}}, \"required\": [\"account_id\"]}}\n</tools>\n\nYour task is to decide which functions are needed and collect missing parameters if necessary.\n\nBased on your analysis, provide your response in one of the following JSON formats:\n1. If no functions are needed:\n```\n{\"response\": \"Your response text here\"}\n```\n2. If functions are needed but some required parameters are missing:\n```\n{\"required_functions\": [\"func_name1\", \"func_name2\", ...], \"clarification\": \"Text asking for missing parameters\"}\n```\n3. If functions are needed and all required parameters are available:\n```\n{\"tool_calls\": [{\"name\": \"func_name1\", \"arguments\": {\"argument1\": \"value1\", \"argument2\": \"value2\"}},... (more tool calls as required)]}\n```"
    },
    {
      "role": "user",
      "content": "what can you do?"
    },
    {
      "role": "assistant",
      "content": "{\"response\": \"I can help you with functions that are defined in the provided XML tags. For example, I can assist with retrieving account balance, transferring funds between accounts, or retrieving account transactions. If you need help with a specific function or have any other questions, please let me know!\"}"
    },
    {
      "role": "user",
      "content": "I want to know balance on my account for account id 11924"
    },
    {
      "role": "assistant",
      "content": "{\"tool_calls\": [{\"name\": \"account_balance\", \"arguments\": {\"account_id\": \"11924\"}}]}"
    },
    {
      "role": "user",
      "content": "<tool_response>\n{'name': 'account_balance', 'result': '150,000 USD'}\n</tool_response>"
    },
    {
      "role": "assistant",
      "content": "{\"response\": \"The account balance for account ID 11924 is 150,000 USD.\"}"
    },
    {
      "role": "user",
      "content": "I want to transfer funds from account 11924 to account 11925"
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
jsonpath "$.choices[0].message.content" matches /required_functions.*transfer_funds.*clarification/
jsonpath "$.choices[0].message.role" == "assistant"
