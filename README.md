# ExoZora Core

## Project Vision
ExoZora is a Rust-based AI-first operating system layer focused on safe orchestration of user intent into controlled system actions.

## Core Principles
- Local-first execution by default.
- Deterministic planning and policy mediation before execution.
- Explicit trust boundaries across system layers.
- Minimal privileged surface area.
- Auditable, modular runtime components.

## Architecture Overview
The foundational control flow is:

`LLM → Planner → Policy → Executor`

- **LLM**: Interprets user intent into structured output.
- **Planner**: Converts intent into explicit task plans.
- **Policy**: Validates plans against safety and permission constraints.
- **Executor**: Runs only approved actions through constrained interfaces.

## Trust Boundaries
- LLM output is treated as untrusted input.
- Planner and policy components run in user-level trust zones.
- Privileged operations are isolated behind tightly scoped interfaces.
- Cross-boundary actions must remain explicit and auditable.

## Current Milestone
**v0.01 – Dev CLI Slice**

This repository is currently in pre-implementation scaffolding to support contributor onboarding and architecture-first development.

## Explicit Non-Goals
At this stage, ExoZora does **not** include:
- Planner implementation logic.
- Policy engine implementation logic.
- Execution layer implementation logic.
- Cloud integrations.
- Live LLM API calls.
- Privileged daemon implementation.
- Containerization runtime setup.
