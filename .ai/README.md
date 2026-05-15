# `.ai/` — Configuración de IA del proyecto

> Esta carpeta agrupa prompts, fixtures y utilidades para que agentes (o
> humanos) iteren sobre la integración con LLMs sin mezclarlos con la
> documentación (`docs/`) ni con el código (`src-tauri/`, `app/`).

## Contenido

```
.ai/
├── README.md                ← este archivo
├── prompts/                 ← prompts del sistema vigentes
│   └── deepseek-structure.md
└── fixtures/                ← textos de prueba e inputs/outputs de referencia
```

## Convenciones

- **Un prompt = un archivo**. El nombre del archivo identifica la versión
  vigente; cambios mayores van como `*-vN.md`. La versión "canónica" sin
  sufijo apunta siempre a la última.
- **Los prompts en Rust** (constantes como
  `services::deepseek_service::SYSTEM_PROMPT`) son la **fuente de
  ejecución**; los archivos en `prompts/` son para revisión y edición
  humana.
- Cuando cambies un prompt:
  1. Editar el archivo en `.ai/prompts/`.
  2. Actualizar la constante en el servicio Rust correspondiente.
  3. Si el cambio es estructural (formato esperado, reglas duras),
     escribir un ADR en `docs/decisions/`.

## Fixtures

`fixtures/` contiene textos reales o sintéticos para probar la
estructuración con DeepSeek. Convención:

- `fixtures/<tema>/input.md` — texto pegado por el usuario.
- `fixtures/<tema>/expected.json` — JSON esperado tras DeepSeek + validación.

Estos fixtures alimentarán tests de integración cuando el `DeepSeekService`
esté implementado (pendiente).

## Qué **no** va aquí

- API keys reales (esas viven en el keyring del SO).
- Datos de proyectos de usuario.
- Logs de telemetría (pendiente; cuando exista, irá a otra carpeta).
