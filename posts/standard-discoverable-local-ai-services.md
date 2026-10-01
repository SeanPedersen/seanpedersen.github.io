---
date: '2026-10-01'
---
# A Standard for Discoverable Local AI Services

One service on your local computer or on a home or office network can serve several AI applications. Serving LLMs, text and image embedding models, speech transcription, image generation and much more.

But [local AI apps](/posts/local-ai-chat-apps) still need too much setup: a server address, port, model name, and knowledge of what each model supports.

A standard should answer two questions: **Where is the service? What can its models do?**

Keep an OpenAI-compatible [API](https://en.wikipedia.org/wiki/API) for inference, then add service discovery and per-model architecture metadata. The names and metadata below are proposals, not an existing standard.

## Discover model inputs and outputs

The OpenAI [models endpoint](https://developers.openai.com/api/reference/resources/models/methods/list) lists model IDs and basic metadata. Extend `/v1/models` with an `architecture` object for each model.

[OpenRouter's model catalog](https://openrouter.ai/api/v1/models) already uses the `architecture` object with `input_modalities` and `output_modalities`. 

For LLMs from OpenRouter:
```json
{
  "id": "aswesome/llm-model",
  "description": "The best LLM on planet earth",
  "architecture": {
    "modality":	"text+image+file->text",
    "input_modalities": ["text", "image", "file"],
    "output_modalities": ["text"],
    "tokenizer": "GPT",
    "instruct_type": null
  }
}
```

Build on that idea with embedding-specific metadata:

```json
{
  "id": "siglip-clip-text-vision",
  "description": "A CLIP text and image embedding model for search",
  "architecture": {
    "input_modalities": ["text", "image"],
    "output_modalities": ["embedding"],
    "embedding": {
      "dimensions": 768,
      "shared_space": true
    }
  }
}
```

An [embedding](https://en.wikipedia.org/wiki/Word_embedding) turns input into a vector for tasks such as similarity search. This example accepts text and images and returns vectors with 768 dimensions.

`shared_space: true` means this model maps its input modalities into the same vector space. A photo app could compare a text query with image embeddings directly.

A text search app could select models with `text` input and `embedding` output. A photo app could require `image` input too. Applications could choose suitable models without hard-coding their names.

The nested `embedding` object is a proposed extension to the OpenRouter-inspired structure. Other model details, such as tool support, can live in separate metadata fields.

Keep familiar inference routes:

```text
POST /v1/chat/completions
POST /v1/embeddings
POST /v1/audio/transcriptions
POST /v1/audio/speech
```

The standard does not need to define every request and response format itself. Instead, the service can publish an [OpenAPI](https://www.openapis.org/) document describing the endpoints and schemas it supports.

```text
GET /v1/openapi.json
```

The OpenAPI document can specify how text, image and audio inputs are represented, which parameters an endpoint accepts, its response schema, error responses, and any service-specific extensions.

This also avoids forcing every local AI server to implement exactly the same set of inference APIs (but that would be nice). Applications or agents can discover the service and its models, then inspect the OpenAPI document when they need the precise wire format for an operation.

## Find the server automatically

A fixed port does not solve discovery. Ports can conflict, and users still need the server's address.

Use [DNS Service Discovery (DNS-SD)](https://en.wikipedia.org/wiki/Zero-configuration_networking#DNS-based_service_discovery) instead. On a local network, it can run over [Multicast DNS (mDNS)](https://en.wikipedia.org/wiki/Multicast_DNS), as described in [RFC 6763](https://www.rfc-editor.org/rfc/rfc6763) and [RFC 6762](https://www.rfc-editor.org/rfc/rfc6762).

A server could advertise this proposed service type:

```text
Service:  _local-ai._tcp.local.
Instance: Desktop AI
Host:     workstation.local.
Port:     43821
```

DNS-SD lets clients browse service instances and resolve each hostname and port. The server can choose any available port.

Keep discovery records small. Put detailed metadata in [HTTP](https://en.wikipedia.org/wiki/HTTP). After discovery, the client could request `GET /.well-known/ai`:

```json
{
  "protocol": "local-ai",
  "version": "1",
  "api": {
    "type": "openai-compatible",
    "base": "/v1"
  }
}
```

The standard would also need to specify transport and authentication, so clients know how to connect before fetching this document.

The full flow stays small:

```text
DNS-SD / mDNS: _local-ai._tcp.local.
    ↓ hostname + port
GET /.well-known/ai
    ↓ protocol + API base
GET /v1/models
    ↓ models + architecture metadata
POST /v1/*
    ↓ inference
```

Managed networks can publish DNS-SD records through unicast DNS in a configured browsing domain. This supports discovery across routed networks where local mDNS does not reach.

**DNS-SD finds the service. The discovery document identifies the protocol. Model architecture describes inputs and outputs. The inference API runs the request.**

Together, these pieces could let applications share local AI infrastructure without asking users to manage server addresses, ports, or model-specific setup.

## References

1. [RFC 6763: DNS-Based Service Discovery](https://www.rfc-editor.org/rfc/rfc6763)
2. [RFC 6762: Multicast DNS](https://www.rfc-editor.org/rfc/rfc6762)
3. [OpenAI API: List models](https://developers.openai.com/api/reference/resources/models/methods/list)
4. [OpenRouter API: Model catalog](https://openrouter.ai/api/v1/models)

#coding #AI
