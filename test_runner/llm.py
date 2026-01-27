"""LLM factory for benchmark."""

import os
import re

from langchain_anthropic import ChatAnthropic
from langchain_core.language_models import BaseChatModel
from langchain_openai import ChatOpenAI
from pydantic import SecretStr

from .models import ModelConfig


def _resolve(value: str | None) -> str | None:
    """Resolve ${VAR} to env value, or return literal."""
    if not value:
        return None
    match = re.match(r"^\$\{(\w+)\}$", value)
    if match:
        result = os.environ.get(match.group(1))
        if not result:
            raise ValueError(f"Environment variable {match.group(1)} not set")
        return result
    return value


def create_chat_model(config: ModelConfig) -> BaseChatModel:
    """Create LangChain chat model from config."""
    api_key = _resolve(config.api_key)
    if not api_key:
        raise ValueError(f"api_key required for model {config.name}")

    secret = SecretStr(api_key)

    if config.provider == "anthropic":
        return ChatAnthropic(model=config.model_id, api_key=secret, temperature=config.temperature)  # type: ignore[call-arg]

    return ChatOpenAI(
        model=config.model_id,
        api_key=secret,
        temperature=config.temperature,
        base_url=_resolve(config.base_url),
    )
