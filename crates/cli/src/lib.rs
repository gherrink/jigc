//! `jigc` library surface — currently the subprocess **probe invoker** the CLI owns
//! ([module-layout.md](../../../implementation/module-layout.md) → Probe boundary). The
//! invoker is exposed as a library item so it can be driven by an integration test
//! against real adversarial stub processes (T2), independent of the `jigc` binary's
//! command dispatch (which wires the invoker into the validate path in T3).
//!
//! The binary (`main.rs`) keeps its own command-dispatch module tree; only the
//! cross-cutting, separately-testable invoker lives here.

pub mod invoke;
