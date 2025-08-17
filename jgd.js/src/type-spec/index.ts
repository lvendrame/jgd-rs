/**
 * Type specification module exports - equivalent to Rust mod.rs
 */

export { ArraySpec, type ArraySpecInput } from "./array-spec";
export { NumberSpec, type NumberSpecInput } from "./number-spec";
export { OptionalSpec, type OptionalSpecInput } from "./optional-spec";
export { FieldGenerator, type FieldSpec } from "./field";
export { Jgd } from "./jgd";

// New structured types that mirror Rust exactly
export { Entity } from "./entity-spec";
export { Jgd as JgdClass } from "./jgd-schema";
export { Count, resolveCount, type CountSpec } from "./count";

// Re-export utils for convenience
export * from "../utils";
