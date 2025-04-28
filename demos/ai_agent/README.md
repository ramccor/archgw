In chat_ui use following .env.local,

MONGODB_URL=mongodb://localhost:27017
MODELS=`[
  {
    "name": "gpt-4o",
    "endpoints": [{
      "type" : "openai",
      "baseURL": "http://localhost:14000/v1"
    }],
  },
]`
