/**
 * Replacer for handling faker patterns and placeholder replacement.
 *
 * This module handles the parsing and execution of faker patterns like "${name.firstName}"
 * and context-aware keys like "${index}", "${count}", etc.
 */

import type {
  Arguments,
  GenerationResult,
  JsonValue,
  GeneratorConfig,
  LocalConfig,
  CustomKeyFunction,
} from "../types";
import {
  success,
  error,
  parseArguments,
  getStringFromArgs,
  getNumberFromArgs,
  getNumberRangeFromArgs,
  getIndexAtDepth,
  getCurrentCount,
  getCurrentEntityName,
  getCurrentFieldName,
} from "./generator-utils";
import { JgdKeyGenerator } from "../fake/fake-keys";

/**
 * Regular expression for matching faker patterns.
 * Matches patterns like: ${name.firstName}, ${lorem.sentence(5,10)}, ${number.int(1,100)}
 */
const FAKER_PATTERN_REGEX = /^\$\{([^}]+)\}$/;

/**
 * Handles placeholder replacement and value generation for JGD patterns.
 */
export class Replacer {
  constructor(
    public readonly pattern: string,
    public readonly args: Arguments = { type: "none" }
  ) {}

  /**
   * Creates a Replacer from a pattern string.
   * Automatically parses arguments from patterns like "${method(arg1,arg2)}"
   */
  static from(pattern: string): Replacer {
    const match = pattern.match(FAKER_PATTERN_REGEX);
    if (!match) {
      // If it's not a faker pattern, return as is but let the generator handle it
      return new Replacer(pattern, { type: "none" });
    }

    const fullPattern = match[1];

    // Check for method with arguments: "method(arg1,arg2)"
    const methodMatch = fullPattern.match(/^([^(]+)\(([^)]*)\)$/);
    if (methodMatch) {
      const [, methodName, argsString] = methodMatch;
      return new Replacer(methodName, parseArguments(argsString));
    }

    // Simple pattern without arguments
    return new Replacer(fullPattern, { type: "none" });
  }

  /**
   * Generates a value based on the pattern and arguments.
   */
  generateValue(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue> {
    // Handle context-aware keys first
    const contextResult = this.handleContextKeys(config, localConfig);
    if (contextResult !== null) {
      return contextResult;
    }

    // Handle custom keys
    const customResult = this.handleCustomKeys(config);
    if (customResult !== null) {
      return customResult;
    }

    // Handle built-in faker patterns
    return this.handleFakerPatterns(config);
  }

  /**
   * Handles context-aware keys like ${index}, ${count}, etc.
   */
  private handleContextKeys(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue> | null {
    switch (this.pattern) {
      case "index": {
        const depth = getNumberFromArgs(this.args);
        try {
          const index =
            depth > 0
              ? getIndexAtDepth(config, depth)
              : getIndexAtDepth(config);
          return success(index);
        } catch (err) {
          return error(
            `Index error: ${err instanceof Error ? err.message : String(err)}`
          );
        }
      }

      case "count":
        return success(getCurrentCount(config));

      case "entity.name":
        return success(getCurrentEntityName(config));

      case "field.name":
        return success(getCurrentFieldName(config));

      default:
        return null; // Not a context key
    }
  }

  /**
   * Handles custom registered keys.
   */
  private handleCustomKeys(
    config: GeneratorConfig
  ): GenerationResult<JsonValue> | null {
    const customFunction = config.customKeys.get(this.pattern);
    if (customFunction) {
      try {
        return customFunction(this.args);
      } catch (err) {
        return error(
          `Custom key error: ${
            err instanceof Error ? err.message : String(err)
          }`
        );
      }
    }
    return null;
  }

  /**
   * Handles built-in JGD key patterns.
   */
  private handleFakerPatterns(
    config: GeneratorConfig
  ): GenerationResult<JsonValue> {
    const jgdGenerator = new JgdKeyGenerator(config.faker);

    // Try JGD key generation first
    if (JgdKeyGenerator.isJgdKey(this.pattern)) {
      return jgdGenerator.generate(this.pattern, this.args);
    }

    // Fallback to faker.js pattern handling for compatibility
    return this.handleLegacyFakerPatterns(config);
  }

  /**
   * Handles legacy faker.js patterns for backward compatibility.
   */
  private handleLegacyFakerPatterns(
    config: GeneratorConfig
  ): GenerationResult<JsonValue> {
    const { faker } = config;

    try {
      // Split pattern into category and method
      const parts = this.pattern.split(".");
      if (parts.length !== 2) {
        // If it's not a dot-separated pattern, it might be a literal or unhandled context key
        // Return it as a literal string for now
        return success(this.pattern);
      }

      const [category, method] = parts;

      // Get the faker category
      const fakerCategory = (faker as any)[category];
      if (!fakerCategory || typeof fakerCategory !== "object") {
        return error(`Unknown faker category: ${category}`);
      }

      // Get the faker method
      const fakerMethod = fakerCategory[method];
      if (typeof fakerMethod !== "function") {
        return error(`Unknown faker method: ${category}.${method}`);
      }

      // Execute the faker method with arguments
      const result = this.executeFakerMethod(fakerMethod, this.args);
      return success(result);
    } catch (err) {
      return error(
        `Faker execution error: ${
          err instanceof Error ? err.message : String(err)
        }`
      );
    }
  }

  /**
   * Executes a faker method with the provided arguments.
   */
  private executeFakerMethod(method: Function, args: Arguments): JsonValue {
    switch (args.type) {
      case "none":
        return method();

      case "fixed": {
        const value = this.parseArgumentValue(args.value);
        return method(value);
      }

      case "range": {
        const min = this.parseArgumentValue(args.min);
        const max = this.parseArgumentValue(args.max);

        // For numeric methods, pass as options object
        if (typeof min === "number" && typeof max === "number") {
          return method({ min, max });
        }

        // For other methods, pass as separate arguments
        return method(min, max);
      }

      default:
        return method();
    }
  }

  /**
   * Parses an argument value to the appropriate type.
   */
  private parseArgumentValue(value: string): JsonValue {
    // Try to parse as number
    const numValue = parseFloat(value);
    if (!isNaN(numValue) && isFinite(numValue)) {
      return numValue;
    }

    // Try to parse as boolean
    if (value.toLowerCase() === "true") return true;
    if (value.toLowerCase() === "false") return false;

    // Return as string
    return value;
  }
}

/**
 * Arguments helper class with typed getter methods.
 */
export class ArgumentsHelper {
  constructor(private args: Arguments) {}

  getString(defaultValue: string = ""): string {
    return getStringFromArgs(this.args, defaultValue);
  }

  getNumber(defaultValue: number = 0): number {
    return getNumberFromArgs(this.args, defaultValue);
  }

  getNumberRange(
    defaultMin: number = 0,
    defaultMax: number = 1
  ): [number, number] {
    return getNumberRangeFromArgs(this.args, defaultMin, defaultMax);
  }

  getBool(defaultValue: boolean = false): boolean {
    const str = this.getString(String(defaultValue)).toLowerCase();
    return str === "true" || str === "1" || str === "yes";
  }

  getStringTuple(
    defaultFirst: string = "",
    defaultSecond: string = ""
  ): [string, string] {
    if (this.args.type === "range") {
      return [this.args.min, this.args.max];
    }
    const value = this.getString(defaultFirst);
    return [value, defaultSecond];
  }
}
