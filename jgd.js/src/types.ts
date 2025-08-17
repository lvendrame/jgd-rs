/**
 * Core types and interfaces for JGD (JSON Generator Definition) TypeScript library.
 *
 * This module contains all the type definitions that mirror the Rust implementation,
 * providing a strongly-typed interface for JSON data generation.
 *
 * NOTE: Major structural changes have been made to match Rust architecture:
 * - EntitySpec interface has been moved to Entity class in type-spec/entity-spec.ts
 * - JgdSchema interface has been moved to Jgd class in type-spec/jgd-schema.ts
 * - CountSpec has been moved to Count class in type-spec/count.ts
 *
 * The legacy interfaces have been removed to avoid conflicts. Import from the specific
 * type-spec modules for the new structured classes.
 */

import type { Faker } from "@faker-js/faker";
import type { Arguments } from "./utils/arguments";

// Re-export Arguments for consistency across the codebase
export type { Arguments };

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
 * Custom key function type.
 */
export type CustomKeyFunction = (
  args: Arguments
) => GenerationResult<JsonValue>;

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
