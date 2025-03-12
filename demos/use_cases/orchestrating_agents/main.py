import json
import os
import random
from typing import Any, Dict, List
from fastapi import FastAPI, Response
from datetime import datetime, date, timedelta, timezone
import logging
from pydantic import BaseModel, Field
from opentelemetry import trace
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


class Choice(BaseModel):
    message: Message


@app.post("/sales")
async def sales_agent(req: ChatCompletionsRequest, res: Response):
    logger.info(f"sales: received messages: {req.messages}")
    return "I am a sales agent, how can I help you?"


@app.post("/issues")
async def issues_agent(req: ChatCompletionsRequest, res: Response):
    logger.info(f"issues: received messages: {req.messages}")
    return "I am a issues agent, how can I help you?"


@app.post("/escalate")
async def escalate_agent(req: ChatCompletionsRequest, res: Response):
    logger.info(f"escalates: received messages: {req.messages}")
    return "You're talking to a human, how can I help you?"
