POST http://localhost:14000/v1/chat/completions
Content-Type: application/json

{
  "messages": [
    {
      "role": "user",
      "content": "hello"
    },
    {
      "role": "assistant",
      "content": "Hello! I'm here to assist you with any questions related to currency conversions and exchange rates. How can I help you today?"
    },
    {
      "role": "user",
      "content": "what is the exchange rate of USD to EUR?"
    }
  ],
  "stream": true
}
