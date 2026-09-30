# Server conformance

This kit probes actual runtime schemas. It does not certify a server from fixture
tests or from its advertised OpenAI API compatibility. **G1 remains open.** No
passing real-server report has been collected for this revision. Local inspection
found llama.cpp build 10034 and an RTX 2080 Ti; vLLM is not installed and Docker's
Linux engine is unavailable. The launch/probe code has fixture validation only.

`pins.json` fixes SmolLM2-135M-Instruct's model revision and UTF-8 Jinja template
digest, vLLM 0.30.0 and llama.cpp's full commit. These are a small conformance model,
not the Study A model pool. In a separate serving environment, install the pinned
server. Obtain `tokenizer_config.json` from the pinned Hugging Face revision and
write the decoded `chat_template` string as UTF-8 **without adding a newline**.
For llama.cpp, convert that same model revision with the pinned converter, retain
its input hashes/conversion command, and supply the resulting GGUF SHA-256. A GGUF
digest alone does not prove which weights were converted. Retain package locks,
GPU/driver details and conversion receipts alongside probe reports.

Example commands (replace paths; vLLM requires its own installed environment):

```sh
python conformance/launch.py vllm --template smollm.jinja --receipt vllm-launch.json
python conformance/launch.py llama_cpp --template smollm.jinja --receipt llama-launch.json --llama-server /path/to/llama-server --gguf /path/to/model.gguf --gguf-sha256 <actual-digest>
```

Both launchers bind loopback. vLLM explicitly selects xgrammar with compact JSON;
llama.cpp disables reasoning. Neither setting is presumed effective before probing.
The launcher checks versions/digests and writes an exclusive, new launch receipt;
it is not a readiness or conformance receipt. Stop one server before reusing its port.

Copy a local experiment template, set the actual model, tokenizer/endpoints and
each agent's `inference` metadata to these pins (`server`: `vllm` / `llama_cpp`,
`grammar_backend`: `xgrammar` / `llama.cpp-json-schema`). For llama.cpp use the full
commit as `server_version`. Set `conformance_report_sha256` to null while probing.
Use empty private state: the contract output is operator-only and includes the
owner's initial notes. Configure the largest population, string limits and peer
ledger the intended experiment will use. Export and probe **both phases**:

```sh
cargo run --locked -- schema --config local.json --owner a --phase selection > selection.json
uv sync --project analysis --locked
uv run --project analysis python conformance/probe.py --base-url http://127.0.0.1:8000 --contract selection.json --launch-receipt vllm-launch.json --output vllm-selection.json
```

Repeat with `--phase communication` and the other backend. PowerShell 7 emits
UTF-8 when redirecting native output; the reader also accepts a UTF-8 BOM.
Reports include raw requests/responses, template/token checks, enum/string
validation, attempted whitespace/reasoning escapes and a forced length finish.
Missing usage, failed requests and invalid responses fail the report. The script
never overwrites an existing report. Review all receipts before admitting G1;
metadata and fingerprints are consistency checks, not authentication.

The proposed reserve is `ceil(1.5 * largest completed response token count)` only
when every probe passes. This is an **empirical sample maximum**, not a bound on
all legal schema outputs. A prompt asking for maximum-length text does not prove
the model filled every field. Inspect those responses and exercise populated
ledgers, all consent branches, escaped Unicode and every intended limit setting.
Do not reduce the current 4096-token examples from a short-output probe. A changed
model, template, grammar or response schema requires new receipts. Commit reviewed
reports and record their SHA-256 in the study manifest and agent provenance.

Primary references (checked 2026-09-28):

- [Pinned vLLM structured output options](https://github.com/vllm-project/vllm/blob/v0.30.0/vllm/config/structured_outputs.py)
- [llama.cpp server at the pinned commit](https://github.com/ggml-org/llama.cpp/tree/505b1ed15ca80e2a19f12ff4ac365e40fb374053/tools/server)
- [Pinned model and tokenizer](https://huggingface.co/HuggingFaceTB/SmolLM2-135M-Instruct/tree/12fd25f77366fa6b3b4b768ec3050bf629380bac)

Offline negative fixtures: `uv run --project analysis python -m unittest discover -s conformance -v`.
