/**
 * Type specification module exports - equivalent to Rust mod.rs
 */

export { ArraySpec } from "./array-spec";
export { NumberSpec } from "./number-spec";
export { OptionalSpec } from "./optional-spec";
export { FieldGenerator, type FieldSpec } from "./field";
export { Jgd, generateFromString, generateFromFile } from "./jgd";

// New structured types that mirror Rust exactly
export { Entity } from "./entity";
export { Count, resolveCount, type CountSpec } from "./count";

// Re-export utils for convenience
export * from "../utils";
