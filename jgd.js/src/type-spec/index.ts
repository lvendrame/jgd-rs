/**
 * Type specification module exports - equivalent to Rust mod.rs
 */

export { ArraySpec, type ArraySpecInput } from "./array-spec";
export { NumberSpec, type NumberSpecInput } from "./number-spec";
export { OptionalSpec, type OptionalSpecInput } from "./optional-spec";
export { FieldGenerator, type FieldSpec } from "./field";
export { EntityGenerator } from "./entity";
export { Jgd } from "./jgd";

// New structured types that mirror Rust exactly
export { Entity, type EntitySpec } from "./entity-spec";
export { Jgd as JgdClass, type JgdSchema } from "./jgd-schema";
export { Count, resolveCount, type CountSpec } from "./count";

// Re-export utils for convenience
export * from "../utils";
