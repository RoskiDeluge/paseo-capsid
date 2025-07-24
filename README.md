# paseo‑capsid

*A tiny Rust runtime + CLI that boots ****Paseo pods**** (data‑first “capsids”) and lets you inspect or mutate their state.*

---

## ✨ What is it?

`paseo‑capsid` is the neutral **shipping container loader** for the Paseo ecosystem.\
Each **pod** is simply a directory containing:

```
my‑pod/
├─ paseo.json   # 3‑field manifest (entity, memory, schema)
└─ state.json   # the mutable genome / memory blob
```

`paseo‑capsid` links the manifest, mounts the memory, and then (soon) injects **enzymes**—pluggable host functions like `echo`, `llm‑chat`, or custom business logic.

For the initial milestone it acts as an **inspector**: load a pod, validate its manifest, print a summary.

---

## 🚀 Quick start

```bash
# 1. clone & build
$ git clone https://github.com/your‑org/paseo‑capsid.git
$ cd paseo‑capsid && cargo build --release

# 2. run against the sample pod
$ cargo run -- ./pods/demo
✅ loaded pod 'Midtown Community Garden'
   • memory → ./state.json
   • schema → paseo://core/v0
```

---

## 🗂️ Pod manifest (paseo.json)

The full spec lives in [`/docs/paseo_manifest_spec.md`](./docs/paseo_manifest_spec.md)\
**Minimum viable:**

```json
{
  "entity": "My Pod",
  "memory": "./state.json",
  "schema": "paseo://core/v0"
}
```

---

## 🛣️ Roadmap (high level)

| Stage    | Focus                                                 |
| -------- | ----------------------------------------------------- |
| **v0.1** | Load & validate manifest, pretty print summary        |
| **v0.2** | JSON‑Schema validation errors with colours (`miette`) |
| **v0.3** | Enzyme registry (`--enzyme echo`) & simple dispatch   |
| **v1.0** | WASM agent loading via Wasmtime Component Model       |

---

## 🤝 Contributing

PRs & issues welcome! Start with `cargo clippy --all-targets --all-features` and follow `rustfmt`.  Open a discussion thread for bigger features.
