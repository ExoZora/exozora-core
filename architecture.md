# ExoZora Core Architecture

**Status:** Draft v0.1  
**Author:** ExoZora Core  
**Created:** 2026-02-18  

---

## 1. Objective
Define the foundational architecture of **ExoZora** as an AI-first operating system layer built on top of Linux. This document establishes structural boundaries, trust zones, and component responsibilities.

## 2. Design Philosophy
ExoZora is built on the following core principles:
* **Local-first execution:** Core logic runs locally by default; cloud is fallback only.
* **Explicit trust boundaries:** Clear separation between unprivileged, privileged, and sandboxed components.
* **Minimal privileged surface area:** Root-level operations are isolated behind a narrow API.
* **Deterministic mediation:** AI does not execute directly; actions pass through structured validation.
* **Modular tool architecture:** Tools are isolated, permission-scoped, and replaceable.
* **Event-driven core:** Components communicate asynchronously via structured messages.
* **Developer-first automation:** Primary focus is optimizing developer productivity workflows.

## 3. High-Level Flow
No component bypasses this chain:

`User (Voice/CLI/UI)` → `LLM Layer` → `Structured Planner` → `Policy Engine` → `Permissioned Executor` → `Tool Runtime/OS`



---

## 4. Core Components

### 4.1 UI Agent
* Runs as a normal user process.
* Handles voice input, CLI interaction, and UI rendering.
* **Restriction:** No elevated privileges; cannot directly execute system-level actions.

### 4.2 LLM Interface Layer
* Translates user intent into structured intermediate representation.
* Supports local models (default) or cloud fallback.
* **Restriction:** No direct filesystem, network, or OS access.

### 4.3 Structured Planner
* Converts intent into a deterministic **task graph**.
* Defines required capabilities and resources.
* **Restriction:** Does not execute tasks; only outputs machine-validated plans.

### 4.4 Policy Engine
* Validates task graphs against system policies (permissions, resource constraints).
* Rejects or modifies unsafe plans.
* Logs all decision points for auditability.

### 4.5 Permissioned Executor
* Executes only policy-approved tasks.
* Delegates tasks to User-level tools, Sandboxed tools, or the Privileged Daemon.

### 4.6 Privileged Daemon
* Runs with elevated privileges with a minimal, strictly defined API.
* Handles system config, package installs, and network changes.
* **Restriction:** No direct AI access; communicates only through validated executor calls.

### 4.7 Tool Runtime
* Isolated environments (Containers/Sandboxes).
* Scoped permissions per tool; no privilege escalation.

### 4.8 Memory Vault
* Encrypted local persistent storage for preferences, system state, and historical plans.
* Access controlled strictly through the Policy Engine.

---

## 5. Trust Boundaries

| Zone | Components | Trust Level |
| :--- | :--- | :--- |
| **Untrusted** | LLM Outputs, External Tools, Plugins | Low |
| **User-Level** | UI Agent, Planner, Policy Engine | Medium |
| **Privileged** | Privileged Daemon | High (Minimal Scope) |

### Isolation Requirements:
1.  **LLM** cannot directly invoke system calls.
2.  **Privileged Daemon** cannot call back into the LLM.
3.  **Tools** cannot access the Daemon unless explicitly mediated.
4.  All cross-boundary communication must be logged.

---

## 6. Non-Goals
ExoZora is **NOT**:
* A full Linux distribution or custom kernel.
* A cloud-dependent assistant.
* A simple chatbot wrapper around shell commands.
* An unrestricted automation framework.

## 7. Open Questions
* **IPC:** Unix sockets, gRPC, or message bus?
* **Sandbox Tech:** Podman containers vs. seccomp vs. custom runtime?
* **Memory:** Strategy for local vector storage for the LLM context.

---
© 2026 ExoZora Core. All rights reserved.
