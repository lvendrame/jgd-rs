/**
 * Utility functions for JGD data generation.
 *
 * This module provides helper functions for parsing, validation, and common
 * operations used throughout the JGD system.
 */

import type { GenerationResult, JsonValue, GeneratorConfig } from "../types";
import { Arguments, ArgumentsType } from "./arguments";

/**
 * Creates a success result.
 */
export function success<T>(data: T): GenerationResult<T> {
  return { success: true, data };
}

/**
 * Creates an error result.
 */
export function error<T = JsonValue>(message: string): GenerationResult<T> {
  return { success: false, error: message };
}

/**
 * Parses arguments from a faker pattern string.
 * For example: "${name.firstName(param1,param2)}" -> Arguments
 */
export function parseArguments(argsString?: string): Arguments {
  if (!argsString || !argsString.trim()) {
    return new Arguments(ArgumentsType.None);
  }

  // Handle range format like "(1,10)" or "(1..10)"
  if (argsString.includes(",") || argsString.includes("..")) {
    return new Arguments(
      ArgumentsType.Range,
      undefined,
      argsString.split(/[,.]/)[0]?.trim() || "",
      argsString.split(/[,.]/)[1]?.trim() || ""
    );
  }

  // Handle fixed value format like "(42)"
  if (argsString.trim()) {
    return new Arguments(ArgumentsType.Fixed, argsString.trim());
  }

  return new Arguments(ArgumentsType.None);
}

/**
 * Gets a string value from Arguments.
 */
export function getStringFromArgs(args: Arguments, defaultValue = ""): string {
  return args.getString(defaultValue);
}

/**
 * Gets a number value from Arguments.
 */
export function getNumberFromArgs(args: Arguments, defaultValue = 0): number {
  return args.getNumber(defaultValue);
}

/**
 * Gets a tuple of numbers from range Arguments.
 */
export function getNumberRangeFromArgs(
  args: Arguments,
  defaultMin = 0,
  defaultMax = 100
): [number, number] {
  return args.getNumberRange(defaultMin, defaultMax);
}

/**
 * Safely increments the depth and adds to index stack.
 */
export function pushDepth(
  config: GeneratorConfig,
  index: number,
  count: number,
  entityName?: string,
  fieldName?: string
): void {
  config.depth++;
  config.indexStack.push(index);
  config.countStack.push(count);

  if (entityName) {
    config.entityNameStack.push(entityName);
  }
  if (fieldName) {
    config.fieldNameStack.push(fieldName);
  }
}

/**
 * Safely decrements the depth and removes from index stack.
 */
export function popDepth(
  config: GeneratorConfig,
  hadEntityName: boolean = false,
  hadFieldName: boolean = false
): void {
  if (config.depth > 0) {
    config.depth--;
    config.indexStack.pop();
    config.countStack.pop();

    if (hadEntityName && config.entityNameStack.length > 0) {
      config.entityNameStack.pop();
    }
    if (hadFieldName && config.fieldNameStack.length > 0) {
      config.fieldNameStack.pop();
    }
  }
}

/**
 * Gets the current index at a specific depth level.
 */
export function getIndexAtDepth(
  config: GeneratorConfig,
  depth?: number
): number {
  if (depth === undefined) {
    // Return current depth index
    return config.indexStack[config.indexStack.length - 1] || 1;
  }

  // Return index at specific depth (1-based)
  const actualDepth = config.indexStack.length - depth + 1;
  if (actualDepth >= 0 && actualDepth < config.indexStack.length) {
    return config.indexStack[actualDepth];
  }

  throw new Error(
    `Invalid depth: ${depth}. Current depth: ${config.indexStack.length}`
  );
}

/**
 * Gets the current count at the current depth level.
 */
export function getCurrentCount(config: GeneratorConfig): number {
  return config.countStack[config.countStack.length - 1] || 1;
}

/**
 * Gets the current entity name.
 */
export function getCurrentEntityName(config: GeneratorConfig): string {
  return config.entityNameStack[config.entityNameStack.length - 1] || "";
}

/**
 * Gets the current field name.
 */
export function getCurrentFieldName(config: GeneratorConfig): string {
  return config.fieldNameStack[config.fieldNameStack.length - 1] || "";
}

/**
 * Validates that a schema has either root or entities but not both.
 */
export function validateSchemaMode(root?: unknown, entities?: unknown): void {
  const hasRoot = root !== undefined && root !== null;
  const hasEntities = entities !== undefined && entities !== null;

  if (hasRoot && hasEntities) {
    throw new Error('Schema cannot have both "root" and "entities" properties');
  }

  if (!hasRoot && !hasEntities) {
    throw new Error('Schema must have either "root" or "entities" property');
  }
}

/**
 * Clones a GeneratorConfig for safe nested generation.
 */
export function cloneConfig(config: GeneratorConfig): GeneratorConfig {
  return {
    ...config,
    customKeys: new Map(config.customKeys),
    indexStack: [...config.indexStack],
    countStack: [...config.countStack],
    entityNameStack: [...config.entityNameStack],
    fieldNameStack: [...config.fieldNameStack],
  };
}

/**
 * Validates a number range specification.
 */
export function validateNumberRange(min: number, max: number): void {
  if (min > max) {
    throw new Error(
      `Invalid range: min (${min}) cannot be greater than max (${max})`
    );
  }

  if (!isFinite(min) || !isFinite(max)) {
    throw new Error(
      `Invalid range: both min (${min}) and max (${max}) must be finite numbers`
    );
  }
}

/**
 * Safely converts a value to a JSON-serializable type.
 */
export function toJsonValue(value: unknown): JsonValue {
  if (value === null || value === undefined) {
    return null;
  }

  if (
    typeof value === "string" ||
    typeof value === "number" ||
    typeof value === "boolean"
  ) {
    return value;
  }

  if (Array.isArray(value)) {
    return value.map(toJsonValue);
  }

  if (typeof value === "object") {
    const result: { [key: string]: JsonValue } = {};
    for (const [key, val] of Object.entries(value)) {
      result[key] = toJsonValue(val);
    }
    return result;
  }

  // For functions, symbols, etc., convert to string
  return String(value);
}
