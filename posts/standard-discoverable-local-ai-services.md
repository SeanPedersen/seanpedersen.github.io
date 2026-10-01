---
date: '2026-10-01'
---

# A Standard for Discoverable Local AI Services

One local AI service on your computer or on a home or office network can serve several AI applications: LLMs, text and image embedding models, speech transcription, image generation, and much more.

But [local AI apps](/posts/local-ai-chat-apps) still need too much setup: a server address, port, model name, and knowledge of what each model supports.

A standard should answer two questions: **Where is the service? What can its models do?**

Publish an OpenAPI document so applications can discover the endpoints, schemas, and API versions exposed by a service. Use the OpenAI models endpoint to describe which models are currently available, what each model can do, and which API schemas apply to it. When the server address is not already known, DNS-based service discovery can make the service discoverable on the local machine and network.

The names and metadata below are proposals, not an existing standard.

## Discover the API

The standard does not need to define every request and response format itself. Instead, a service can publish an [OpenAPI](https://www.openapis.org/) document describing the endpoints and schemas it exposes.

```text
GET /openapi.json
```

The OpenAPI document describes how the server API is invoked: which paths and HTTP methods exist, how text, image, audio, files, and other data are represented, which parameters an operation accepts, its response schemas, error responses, service-specific extensions, and any API versions exposed by the service.

It defines the exact request and response schemas used by each API operation. It does not need to list which models are currently loaded or available. That is done by the models endpoint.

For example, a simplified OpenAPI document could define reusable modality schemas alongside OpenAI-compatible chat-completion and embedding operations:

```json
{
  "openapi": "3.1.0",
  "info": {
    "title": "Local AI API",
    "version": "1.0.0"
  },

  "components": {
    "schemas": {
      "TextContentPart": {
        "type": "object",
        "properties": {
          "type": {
            "const": "text"
          },
          "text": {
            "type": "string"
          }
        },
        "required": ["type", "text"]
      },

      "ImageContentPart": {
        "type": "object",
        "properties": {
          "type": {
            "const": "image"
          },
          "image": {
            "type": "string",
            "format": "uri"
          }
        },
        "required": ["type", "image"]
      },

      "FileContentPart": {
        "type": "object",
        "properties": {
          "type": {
            "const": "file"
          },
          "file": {
            "type": "string",
            "format": "uri"
          }
        },
        "required": ["type", "file"]
      },

      "TextOutput": {
        "type": "string"
      },

      "EmbeddingTextInput": {
        "type": "string"
      },

      "EmbeddingImageInput": {
        "type": "string",
        "format": "uri"
      },

      "EmbeddingOutput768": {
        "type": "array",
        "items": {
          "type": "number"
        },
        "minItems": 768,
        "maxItems": 768
      },

      "ChatMessage": {
        "type": "object",
        "properties": {
          "role": {
            "type": "string",
            "enum": ["system", "developer", "user", "assistant", "tool"]
          },
          "content": {
            "oneOf": [
              {
                "type": "string"
              },
              {
                "type": "array",
                "items": {
                  "oneOf": [
                    {
                      "$ref": "#/components/schemas/TextContentPart"
                    },
                    {
                      "$ref": "#/components/schemas/ImageContentPart"
                    },
                    {
                      "$ref": "#/components/schemas/FileContentPart"
                    }
                  ]
                }
              }
            ]
          }
        },
        "required": ["role", "content"]
      },

      "ChatCompletionRequest": {
        "type": "object",
        "properties": {
          "model": {
            "type": "string"
          },
          "messages": {
            "type": "array",
            "items": {
              "$ref": "#/components/schemas/ChatMessage"
            },
            "minItems": 1
          },
          "temperature": {
            "type": "number"
          },
          "stream": {
            "type": "boolean",
            "default": false
          }
        },
        "required": ["model", "messages"]
      },

      "ChatCompletionResponse": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string"
          },
          "object": {
            "type": "string",
            "const": "chat.completion"
          },
          "created": {
            "type": "integer"
          },
          "model": {
            "type": "string"
          },
          "choices": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "index": {
                  "type": "integer"
                },
                "message": {
                  "$ref": "#/components/schemas/ChatMessage"
                },
                "finish_reason": {
                  "type": ["string", "null"]
                }
              },
              "required": ["index", "message", "finish_reason"]
            }
          }
        },
        "required": ["id", "object", "created", "model", "choices"]
      },

      "TextEmbeddingRequest": {
        "type": "object",
        "properties": {
          "model": {
            "type": "string"
          },
          "input": {
            "$ref": "#/components/schemas/EmbeddingTextInput"
          }
        },
        "required": ["model", "input"]
      },

      "ImageEmbeddingRequest": {
        "type": "object",
        "properties": {
          "model": {
            "type": "string"
          },
          "input": {
            "$ref": "#/components/schemas/EmbeddingImageInput"
          }
        },
        "required": ["model", "input"]
      }
    }
  },

  "paths": {
    "/v1/chat/completions": {
      "post": {
        "operationId": "createChatCompletion",
        "requestBody": {
          "required": true,
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/ChatCompletionRequest"
              }
            }
          }
        },
        "responses": {
          "200": {
            "description": "Chat completion",
            "content": {
              "application/json": {
                "schema": {
                  "$ref": "#/components/schemas/ChatCompletionResponse"
                }
              }
            }
          }
        }
      }
    },

    "/v1/embeddings": {
      "post": {
        "operationId": "createEmbedding",
        "requestBody": {
          "required": true,
          "content": {
            "application/json": {
              "schema": {
                "oneOf": [
                  {
                    "$ref": "#/components/schemas/TextEmbeddingRequest"
                  },
                  {
                    "$ref": "#/components/schemas/ImageEmbeddingRequest"
                  }
                ]
              }
            }
          }
        },
        "responses": {
          "200": {
            "description": "Embedding vector",
            "content": {
              "application/json": {
                "schema": {
                  "$ref": "#/components/schemas/EmbeddingOutput768"
                }
              }
            }
          }
        }
      }
    }
  }
}
```

This example is intentionally simplified. A real OpenAI-compatible chat schema can support more message types, tools, structured output, streaming, audio, and other parameters.

The important part is that the OpenAPI document does more than define reusable schemas. It also declares **which operations consume and produce those schemas**.

Applications can therefore resolve a model's schema references against the OpenAPI document to find the operations that support those input and output types. There is no need to repeat an inference URL or operation identifier in every model entry.

For example, the OpenAPI document tells a client that `/v1/chat/completions` accepts `ChatCompletionRequest`, which in turn can contain `TextContentPart`, `ImageContentPart`, and `FileContentPart`.

Likewise, `/v1/embeddings` accepts request schemas containing `EmbeddingTextInput` or `EmbeddingImageInput` and produces `EmbeddingOutput768`.

This avoids forcing every local AI server to implement exactly the same set of inference APIs, although broad compatibility would still be desirable. Applications or agents can inspect the OpenAPI document when they need the precise wire format for a particular operation.

Keep familiar inference routes where possible:

```text
POST /v1/chat/completions
POST /v1/embeddings
POST /v1/audio/transcriptions
POST /v1/audio/speech
```

A future version could expose different routes under `/v2` without changing the stable discovery location at `/openapi.json`.

## Discover models with inputs and outputs

The OpenAI [models endpoint](https://developers.openai.com/api/reference/resources/models/methods/list) lists model IDs and basic metadata. Extend `/v1/models` with an `architecture` object for each model and references to the OpenAPI schemas that describe the modalities it accepts and produces.

[OpenRouter's model catalog](https://openrouter.ai/api/v1/models) already uses an `architecture` object with fields such as `input_modalities` and `output_modalities`.

For example, a multimodal LLM could be described as:

```json
{
  "id": "awesome/llm-model",
  "description": "A multimodal language model",
  "architecture": {
    "modality": "text+image+file->text",
    "input_modalities": ["text", "image", "file"],
    "output_modalities": ["text"],
    "tokenizer": "GPT",
    "instruct_type": null
  },
  "input_schemas": {
    "text": "/openapi.json#/components/schemas/TextContentPart",
    "image": "/openapi.json#/components/schemas/ImageContentPart",
    "file": "/openapi.json#/components/schemas/FileContentPart"
  },
  "output_schemas": {
    "text": "/openapi.json#/components/schemas/TextOutput"
  }
}
```

The `architecture` object says what the model can conceptually consume and produce.

The schema references say exactly how those modalities are represented by the API.

OpenAPI then describes how those modality schemas are assembled into an actual request. In this example, `TextContentPart`, `ImageContentPart`, and `FileContentPart` can appear inside a `ChatCompletionRequest`, and `/v1/chat/completions` accepts that request schema.

The same pattern can be used for embedding models:

```json
{
  "id": "siglip-clip-text-vision",
  "description": "A text and image embedding model for search",
  "architecture": {
    "input_modalities": ["text", "image"],
    "output_modalities": ["embeddings"]
  },
  "input_schemas": {
    "text": "/openapi.json#/components/schemas/EmbeddingTextInput",
    "image": "/openapi.json#/components/schemas/EmbeddingImageInput"
  },
  "output_schemas": {
    "embeddings": "/openapi.json#/components/schemas/EmbeddingOutput768"
  },
  "embeddings": {
    "shared_space": true
  }
}
```

Schema references are URI references resolved relative to the service origin. Their fragments identify schemas inside the OpenAPI document.

An [embedding](https://en.wikipedia.org/wiki/Word_embedding) turns an input into a vector for tasks such as similarity search.

Here, the model entry says that text input uses the `EmbeddingTextInput` schema, image input uses the `EmbeddingImageInput` schema, and the model produces vectors described by `EmbeddingOutput768`.

Possible optional extra fields for "embeddings" key: matryoshka_dims: list<int>, type: sparse / dense, similarity: L2 / cosine / dot

The OpenAPI document tells the application how those schemas are used by actual operations:

```text
model accepts image
    ↓
image → EmbeddingImageInput
    ↓
inspect /openapi.json
    ↓
ImageEmbeddingRequest contains EmbeddingImageInput
    ↓
POST /v1/embeddings accepts ImageEmbeddingRequest
    ↓
invoke the operation with this model
```

The model metadata itself only needs to describe capabilities and semantics that are specific to the model.

`shared_space: true` means this model maps its supported input modalities into the same vector space. A photo app could therefore compare a text query with image embeddings directly.

A text search app could select models with `text` input and `embeddings` output. A photo app could additionally require `image` input. Applications could choose suitable models without hard-coding their names.

The `embeddings` object is a proposed extension to the OpenRouter-inspired structure. Other model-specific details, such as tool support, context limits, or embedding semantics, can live in separate metadata fields.

The distinction between `/openapi.json` and `/v1/models` is useful:

```text
/openapi.json
    Which API operations exist?
    Which paths and HTTP methods invoke them?
    How are text, image, audio, files, and other modalities represented?
    How are those schemas assembled into requests and responses?

/v1/models
    Which models are available right now?
    What can each model do?
    Which OpenAPI modality schemas apply to that model?
```

Loading or unloading a model only needs to change `/v1/models` if the model uses schemas already defined by the API.

If a new model requires a modality representation, request format, response format, or operation that the API does not yet describe, both documents change: `/openapi.json` defines the new schema or operation, and `/v1/models` references the applicable schemas for that model.

This keeps the API description relatively stable while allowing the model catalog to reflect the server's current runtime inventory.

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
  "openapi": "/openapi.json"
}
```

The standard would also need to specify transport and authentication, so clients know how to connect before fetching this document.

The full flow stays small:

```text
DNS-SD / mDNS: _local-ai._tcp.local.
    ↓ hostname + port

GET /.well-known/ai
    ↓ protocol + OpenAPI location

GET /openapi.json
    ↓ operations + modality schemas + request/response schemas

GET /v1/models
    ↓ current models + capabilities + schema references

POST /v1/*
    ↓ inference
```

Managed networks can publish DNS-SD records through unicast DNS in a configured browsing domain. This supports discovery across routed networks where local mDNS does not reach.

**DNS-SD finds the service. The discovery document identifies the protocol. The OpenAPI document defines the API operations and exact wire schemas. The models endpoint identifies the models currently available, their capabilities, and the schemas that apply to them. The inference API runs the request.**

Together, these pieces could let applications share local AI infrastructure without asking users to manage server addresses, ports, or model-specific setup.

## References

1. [RFC 6763: DNS-Based Service Discovery](https://www.rfc-editor.org/rfc/rfc6763)
2. [RFC 6762: Multicast DNS](https://www.rfc-editor.org/rfc/rfc6762)
3. [OpenAPI Specification](https://spec.openapis.org/oas/v3.1)
4. [OpenAI API: Models](https://developers.openai.com/api/reference/resources/models/methods/list)
5. [OpenRouter API: Model catalog](https://openrouter.ai/api/v1/models)

#coding #AI
