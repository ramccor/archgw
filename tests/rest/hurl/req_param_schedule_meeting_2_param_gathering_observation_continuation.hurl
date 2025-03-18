POST http://localhost:8282/v1/chat/completions
Content-Type: application/json

{
  "model": "Arch-Function",
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant designed to assist with the user query by making one or more function calls if needed.\n\nYou are provided with function signatures within <tools></tools> XML tags:\n<tools>\n{\"name\": \"schedule_meeting\", \"description\": \"Schedule a meeting\", \"parameters\": {\"type\": \"object\", \"properties\": {\"subject\": {\"type\": \"string\", \"description\": \"The subject of the meeting\"}, \"start_date\": {\"type\": \"string\", \"description\": \"The start date of the meeting\"}, \"start_time\": {\"type\": \"string\", \"description\": \"The start time of the meeting\"}, \"participants\": {\"type\": \"array\", \"items\": {\"type\": \"string\"}, \"description\": \"Email addresses of participants\"}, \"duration\": {\"type\": \"string\", \"description\": \"The duration of the meeting\", \"default\": \"1 hour\"}, \"location\": {\"type\": \"string\", \"description\": \"The location of the meeting\"}}, \"required\": [\"subject\", \"start_date\", \"start_time\", \"participants\"]}}\n</tools>\n\nYour task is to decide which functions are needed and collect missing parameters if necessary.\n\nBased on your analysis, provide your response in one of the following JSON formats:\n1. If no functions are needed:\n```\n{\"response\": \"Your response text here\"}\n```\n2. If functions are needed but some required parameters are missing:\n```\n{\"required_functions\": [\"func_name1\", \"func_name2\", ...], \"clarification\": \"Text asking for missing parameters\"}\n```\n3. If functions are needed and all required parameters are available:\n```\n{\"tool_calls\": [{\"name\": \"func_name1\", \"arguments\": {\"argument1\": \"value1\", \"argument2\": \"value2\"}},... (more tool calls as required)]}\n```"
    },
    {
      "role": "user",
      "content": "I need help in scheduling a meeting"
    },
    {
      "role": "assistant",
      "content": "Could you please provide me with the subject of the meeting, the start date and time, and the email addresses of the participants?"
    },
    {
      "role": "user",
      "content": "subject is 'Project Kickoff', start date is tomorrow, start time is 10am, participants are joe@gmail.com and jane@gmail.com"
    },
    {
      "role": "user",
      "content": "<tool_response>\n{'name': 'schedule_meeting', 'result': 'success'}\n</tool_response>"
    },
    {
      "role": "assistant",
      "content": "The meeting was scheduled successfully."
    },
    {
      "role": "user",
      "content": "I want to schedule another meeting"
    },
    {
      "role": "assistant",
      "content": "Could you please provide me with the subject, start date and time, and the email addresses of the participants for the new meeting?"
    },
    {
      "role": "user",
      "content": "Sure, subject is 'Project Kickoff'"
    },
    {
      "role": "assistant",
      "content": "Could you please specify the start date and time for the second meeting?"
    },
    {
      "role": "user",
      "content": "start date is tomorrow, start time is 10am"
    },
    {
      "role": "assistant",
      "content": "Could you please confirm the email addresses of the participants for the second meeting?"
    },
    {
      "role": "user",
      "content": "participants are tim@gmail.com and dave@gmail.com"
    },
    {
        "role": "assistant",
        "content": "Please confirm the details: scheduling a meeting titled 'Project Kickoff' for tomorrow at 10am with participants 'tim@gmail.com' and 'dave@gmail.com'."
    },
    {
      "role": "user",
      "content": "Yes please go ahead"
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
jsonpath "$.choices[0].message.content" matches /tool_response.*schedule_meeting/
jsonpath "$.choices[0].message.role" == "assistant"
