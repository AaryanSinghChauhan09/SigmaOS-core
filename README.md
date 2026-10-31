# SigmaOS-core

Core platform boundary for SigmaOS: boot integration, kernel/user ABI, init,
and base-system contracts. Keep desktop policy and applications out of this
repository.

## Current state

`COMPONENT.toml` marks this repository as a scaffold. The tracked content is
the component manifest, license, README, and a metadata validation workflow;
there is no boot, kernel ABI, init, or base-system implementation here yet.

The main [SigmaOS integration repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
remains the integration and release authority. Define each core contract there
before creating implementations here so ownership and compatibility can be
reviewed against the integrated system.

## Readiness evidence

The validation workflow checks required metadata fields, the `main` branch
declaration, allowed status values, and README presence. It does not verify
bootability, kernel interfaces, or runtime behavior.
