# ⚡ Guiding Light AI

> **Rust CLI that transforms your values into enforceable policies: Git hooks, decision journals, and optional LLM reflection via Ollama.**

[![Rust](https://img.shields.io/badge/Rust-1.70+-DEA584?style=flat&logo=rust&labelColor=0D1117)]()
[![Ollama](https://img.shields.io/badge/LLM-Ollama-FF6B6B?style=flat&labelColor=0D1117)]()
[![License](https://img.shields.io/badge/License-MIT-00FF88?style=flat&labelColor=0D1117)]()

---

## 🎯 What It Does

Guiding Light AI bridges the gap between **values and action**. Define your principles, and this tool enforces them through:

- **Git Hooks** — Block commits that violate your policies
- **Decision Journal** — Log and review decisions against your values
- **LLM Reflection** — Ollama-powered analysis of alignment

## 🏗️ Architecture

```
Values Definition → Policy Engine → Git Hooks + Journal + LLM
```

## 🚀 Quick Start

```bash
# Install
cargo install guiding-light-ai

# Initialize
gl-init

# Configure values
gl-config --edit

# Install hooks
gl-hooks install

# Review decision
gl-decide "Should we use this third-party library?"
```

## 🔧 Features

- ✅ **Pre-commit Hooks** — Automatic policy enforcement
- ✅ **Decision Logging** — Structured journal with search
- ✅ **LLM Reflection** — Ollama integration for deep analysis
- ✅ **Custom Policies** — YAML-based rule definitions
- ✅ **Team Sharing** — Export/import policy sets

## 📄 License

MIT License

---

[🗽 CIVWATCH](https://github.com/POWDER-RANGER/CIVWATCH) | [🏛️ OBLISK](https://github.com/POWDER-RANGER/OBLISK) | [🤖 CharlesAI](https://github.com/POWDER-RANGER/CharlesAI)
