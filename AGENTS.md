# BOAF Agent Engineering Instructions

## 1. Project Identity

BOAF (Buffer Overflow Automated Framework) is a specialised framework for buffer overflow research, testing, fuzzing and analysis.

Its scope must remain strictly limited to its designated purpose. Do not transform BOAF into a general-purpose cybersecurity framework.

## 2. Minimal Code Philosophy

**Implement only what is necessary. Nothing more.**

Every line of code must have a clear technical justification.

Before introducing code, ask:

- Is this implementation necessary for an existing requirement?
- Does an equivalent implementation already exist?
- Can the requirement be fulfilled with less code?
- Does this introduce unnecessary dependencies or abstractions?
- Does this functionality belong within BOAF's scope?

If a line, function, module, dependency or abstraction provides no necessary functionality, safety, reliability or maintainability benefit, do not introduce it.

## 3. Strict Project Scope

### Permitted functionality

- Buffer overflow testing and fuzzing.
- Controlled payload generation and management.
- Stack-based buffer overflow research.
- Heap-based buffer overflow research.
- Input boundary testing.
- Memory corruption and crash observation.
- Test execution and worker management.
- Evidence collection and reporting.
- Platform compatibility for supported operating systems.

### Prohibited scope expansion

Do not introduce unrelated functionality, including:

- General-purpose network scanners.
- Web application vulnerability scanners.
- OSINT and reconnaissance tools.
- Password or credential attack modules.
- General-purpose exploitation frameworks.
- Unrelated penetration testing utilities.
- Features that do not directly support BOAF's designated purpose.

## 4. Architecture and Code Quality

- Follow modular software engineering practices.
- Each module should ideally remain below 250 LOC.
- Exceeding 250 LOC requires justification. 300 LOC is a rare exception.
- Maintain clear separation of responsibilities.
- Avoid unnecessary traits, interfaces, wrappers and abstractions.
- Avoid duplicate implementations.
- Remove dead code and unused dependencies.
- Prefer Rust standard library functionality where sufficient.
- Do not introduce frameworks or dependencies without a demonstrated requirement.

## 5. Security and Reliability

Minimal code must never come at the expense of essential safety.

- Validate inputs and enforce resource limits.
- Handle errors explicitly.
- Avoid uncontrolled resource consumption.
- Support cooperative cancellation and execution timeouts.
- Keep testing within explicitly authorised and configured targets.
- Do not interpret a connection failure as definitive proof of a buffer overflow.
- Do not claim vulnerability detection without sufficient evidence.

## 6. Testing Requirements

- Test actual functionality, not hypothetical features.
- Include relevant unit and integration tests.
- Test expected failure conditions.
- Validate platform-specific behaviour.
- Remove unnecessary or redundant tests only after ensuring required coverage remains intact.

## 7. Platform Support

Initial supported platforms:

- Windows
- Linux

Android/Termux and macOS are outside the initial implementation scope.

Do not introduce platform-specific code for unsupported platforms unless explicitly approved.

## 8. Dependency Policy

Before adding a dependency:

1. Establish the exact requirement.
2. Check whether Rust's standard library can fulfil it.
3. Check whether an existing dependency already provides the functionality.
4. Evaluate maintenance, security and compatibility.
5. Add only if technically justified.

## 9. Agent Behaviour

All coding agents must:

- Read and follow this document before modifying the codebase.
- Inspect existing implementations before creating new ones.
- Avoid unnecessary refactoring.
- Avoid speculative features.
- Avoid changing unrelated modules.
- Preserve existing working functionality.
- Explain why a new dependency or abstraction is necessary.
- Report what was changed and which tests were executed.
- Never claim a feature is implemented or tested when it is not.

If a requirement conflicts with this document, stop and request clarification rather than silently expanding the project scope.

## 10. Definition of Done

An implementation is complete only when:

- It fulfils an approved BOAF requirement.
- It remains within the project's designated scope.
- It handles relevant errors and edge cases.
- Appropriate tests pass.
- No unnecessary dependencies or duplicate logic are introduced.
- Documentation accurately reflects actual behaviour.
- No unrelated functionality is added.

## Fundamental Engineering Principle

**Every line must justify its existence.**

BOAF is not intended to become a large collection of unrelated security tools. Its engineering quality will be measured by how effectively and reliably it performs its specific purpose, not by the number of features or lines of code.

**Build only what BOAF needs. Nothing extra.**
