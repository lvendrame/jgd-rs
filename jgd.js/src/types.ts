/**
 * Core types and interfaces for JGD (JSON Generator Definition) TypeScript library.
 *
 * This module contains all the type definitions that mirror the Rust implementation,
 * providing a strongly-typed interface for JSON data generation.
 */

import type { Faker } from "@faker-js/faker";

/**
 * Represents any JSON-serializable value.
 */
export type JsonValue =
  | string
  | number
  | boolean
  | null
  | JsonValue[]
  | { [key: string]: JsonValue };

/**
 * Result type for generation operations.
 */
export type GenerationResult<T = JsonValue> =
  | {
      success: true;
      data: T;
    }
  | {
      success: false;
      error: string;
    };

/**
 * Arguments passed to custom key functions and pattern generators.
 */
export type Arguments =
  | { type: "none" }
  | { type: "fixed"; value: string }
  | { type: "range"; min: string; max: string };

/**
 * Custom key function type.
 */
export type CustomKeyFunction = (
  args: Arguments
) => GenerationResult<JsonValue>;

/**
 * Count specification for generating multiple items.
 */
export type CountSpec =
  | number
  | { fixed: number }
  | { range: [number, number] }
  | [number, number]; // Support array format from Rust examples

/**
 * Input specification for number generation.
 */
export interface NumberSpecInput {
  min: number;
  max: number;
  integer?: boolean;
}

/**
 * Input specification for array generation.
 */
export interface ArraySpecInput {
  count: CountSpec;
  of: string | number | boolean | NumberSpecInput | { ref: string };
}

/**
 * Input specification for optional fields.
 */
export interface OptionalSpecInput<T = unknown> {
  optional: {
    prob?: number; // 0.0 to 1.0, defaults to 0.5
    of: T;
  };
}

/**
 * Field definition within an entity.
 */
export type FieldSpec =
  | string
  | number
  | boolean
  | NumberSpecInput
  | ArraySpecInput
  | OptionalSpecInput
  | EntitySpec
  | { ref: string }
  | { array: ArraySpecInput };

/**
 * Entity specification for generating complex objects.
 */
export interface EntitySpec {
  count?: CountSpec;
  fields: Record<string, FieldSpec>;
}

/**
 * Root JGD schema definition.
 */
export interface JgdSchema {
  $format: string;
  version: string;
  seed?: number;
  defaultLocale?: string;
  root?: EntitySpec;
  entities?: Record<string, EntitySpec>;
}

/**
 * Configuration for data generation.
 */
export interface GeneratorConfig {
  locale: string;
  seed?: number;
  faker: Faker;
  customKeys: Map<string, CustomKeyFunction>;
  depth: number;
  indexStack: number[];
  countStack: number[];
  entityNameStack: string[];
  fieldNameStack: string[];
}

/**
 * Local configuration for context-aware generation.
 */
export interface LocalConfig {
  entityName?: string;
  fieldName?: string;
  index?: number;
  count?: number;
}

/**
 * Interface for all JSON generators.
 */
export interface JsonGenerator<T = JsonValue> {
  generate(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<T>;
}

/**
 * Error types for generation failures.
 */
export class JgdGeneratorError extends Error {
  constructor(message: string, public readonly cause?: unknown) {
    super(message);
    this.name = "JgdGeneratorError";
  }
}

/**
 * Helper type for ensuring required fields are present.
 */
export type RequiredFields<T, K extends keyof T> = T & Required<Pick<T, K>>;

/**
 * Type guard to check if a value is a GenerationResult success.
 */
export function isSuccess<T>(
  result: GenerationResult<T>
): result is { success: true; data: T } {
  return result.success;
}

/**
 * Type guard to check if a value is a GenerationResult error.
 */
export function isError<T>(
  result: GenerationResult<T>
): result is { success: false; error: string } {
  return !result.success;
}

/**
 * Utility type for extracting the data type from a GenerationResult.
 */
export type ExtractData<T> = T extends GenerationResult<infer U> ? U : never;
