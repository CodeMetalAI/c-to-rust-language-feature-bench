"""LLM factory for benchmark."""

from __future__ import annotations

import os
import re
from dataclasses import dataclass

import anthropic
import openai

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


class ProviderError(RuntimeError):
    """Raised when the LLM provider returns no usable response."""


@dataclass
class ChatResponse:
    """Minimal response wrapper matching the content attribute pattern."""

    content: str


class ChatModel:
    """Thin wrapper providing a unified invoke() interface over anthropic/openai SDKs."""

    def __init__(
        self,
        provider: str,
        model: str,
        api_key: str,
        temperature: float,
        base_url: str | None = None,
    ):
        self._provider = provider
        self._model = model
        self._temperature = temperature
        if provider == "anthropic":
            self._client: anthropic.Anthropic | openai.OpenAI = anthropic.Anthropic(api_key=api_key)
        else:
            self._client = openai.OpenAI(api_key=api_key, base_url=base_url)

    def invoke(self, messages: list[dict[str, str]]) -> ChatResponse:
        """Send messages and return a ChatResponse."""
        if self._provider == "anthropic":
            system_msg = ""
            user_msgs = []
            for m in messages:
                if m["role"] == "system":
                    system_msg = m["content"]
                else:
                    user_msgs.append(m)
            resp = self._client.messages.create(  # type: ignore[union-attr]
                model=self._model,
                max_tokens=16384,
                temperature=self._temperature,
                system=system_msg,
                messages=user_msgs,  # type: ignore[arg-type]
            )
            text_blocks = [block for block in resp.content if hasattr(block, "text")]
            return ChatResponse(content=text_blocks[0].text if text_blocks else "")

        resp = self._client.chat.completions.create(  # type: ignore[union-attr]
            model=self._model,
            temperature=self._temperature,
            messages=messages,  # type: ignore[arg-type]
        )
        choices = resp.choices
        if not choices:
            raise ProviderError("Provider returned no choices")
        return ChatResponse(content=choices[0].message.content or "")


def create_chat_model(config: ModelConfig, temperature: float) -> ChatModel:
    """Create a ChatModel from config."""
    api_key = _resolve(config.api_key)
    if not api_key:
        raise ValueError(f"api_key required for model {config.name}")
    return ChatModel(
        provider=config.provider,
        model=config.model_id,
        api_key=api_key,
        temperature=temperature,
        base_url=_resolve(config.base_url),
    )
