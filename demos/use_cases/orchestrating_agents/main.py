import json
import os
import random
import time
from typing import Any, Dict, List
from fastapi import FastAPI, Response
from datetime import datetime, date, timedelta, timezone
import logging
import openai
from pydantic import BaseModel, Field
from opentelemetry import trace
from fastapi.responses import StreamingResponse
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.instrumentation.fastapi import FastAPIInstrumentor
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.sdk.resources import Resource


resource = Resource.create(
    {
        "service.name": "llm-agents",
    }
)

# Initialize the tracer provider
trace.set_tracer_provider(TracerProvider(resource=resource))
tracer = trace.get_tracer(__name__)

logger = logging.getLogger("uvicorn.error")
logger.setLevel(logging.INFO)

app = FastAPI()
FastAPIInstrumentor().instrument_app(app)

# Configure the OTLP exporter (Jaeger, Zipkin, etc.)
otlp_exporter = OTLPSpanExporter(
    endpoint=os.getenv("OLTP_HOST", "http://localhost:4317")
)
trace.get_tracer_provider().add_span_processor(BatchSpanProcessor(otlp_exporter))


@app.get("/healthz")
async def healthz():
    return {"status": "ok"}


class Message(BaseModel):
    role: str
    content: str


class ChatCompletionsRequest(BaseModel):
    messages: List[Message]
    model: str
    metadata: Dict[str, Any] = None
    stream: bool = False


class Choice(BaseModel):
    message: Message


class ChatCompletionResponse(BaseModel):
    choices: List[Choice]
    metadata: Dict[str, Any] = None


class ChunkChoice(BaseModel):
    delta: Message


class ChatCompletionStreamResponse(BaseModel):
    model: str
    choices: List[ChunkChoice]


openai_client = openai.OpenAI(
    base_url="http://host.docker.internal:12000/v1", api_key="--"
)

agents_definition = {
    "sales_agent": {
        "role": "sales agent",
        "instructions": "You are a sales agent for ACME Inc."
        "Always answer in a sentence or less."
        "Follow the following routine with the user:"
        "1. Ask them about any problems in their life related to catching roadrunners.\n"
        "2. Casually mention one of ACME's crazy made-up products can help.\n"
        " - Don't mention price.\n"
        "3. Once the user is bought in, drop a ridiculous price.\n"
        "4. Only after everything, and if the user says yes, "
        "tell them a crazy caveat and execute their order.\n",
    },
    "issues_and_repairs": {
        "role": "issues and repairs agent",
        "instructions": "You are a customer support agent for ACME Inc."
        "Always answer in a sentence or less."
        "Follow the following routine with the user:"
        "1. First, ask probing questions and understand the user's problem deeper.\n"
        " - unless the user has already provided a reason.\n"
        "2. Propose a fix (make one up).\n"
        "3. ONLY if not satisfied, offer a refund.\n"
        "4. If accepted, search for the ID and then execute refund.",
    },
    "escalate_to_human": {
        "role": "human agent",
        "instructions": """Pretend you are a human trying to address the user's problem.""",
    },
    "unknown agent": {
        "role": "llm agent",
        "instructions": "You are a helpful LLM agent.",
    },
}


def construct_llm_messages(agent_name, messages):
    agent_role = agents_definition.get(agent_name)["role"]
    agent_instructions = agents_definition.get(agent_name)["instructions"]
    system_prompt = "You are a " + agent_role + ". " + agent_instructions

    updated_messages = [{"role": "system", "content": system_prompt}]
    for message in messages:
        updated_messages.append({"role": message.role, "content": message.content})
    return updated_messages


@app.post("/v1/chat/completions")
async def completion_api(req: ChatCompletionsRequest):
    logger.info(f"request: {req}")
    if req.metadata is None:
        req.metadata = {}
    agent_name = req.metadata.get("agent-name", "unknown agent")
    logger.info(f"agent: {agent_name}")
    messages = construct_llm_messages(agent_name, req.messages)
    logger.info("messages: " + str(messages))
    completion = openai_client.chat.completions.create(
        model="None",
        messages=messages,
        stream=req.stream,
    )

    if req.stream:

        def stream():
            for line in completion:
                if line.choices and len(line.choices) > 0 and line.choices[0].delta:
                    chunk_response_str = json.dumps(line.model_dump())
                    yield "data: " + chunk_response_str + "\n\n"
            yield "data: [DONE]" + "\n\n"

        return StreamingResponse(stream(), media_type="text/event-stream")

    else:
        return completion
