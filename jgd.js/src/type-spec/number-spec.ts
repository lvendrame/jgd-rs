/**
 * Number specification generator for creating numeric values within specified ranges.
 */

import type {
  JsonValue,
  GenerationResult,
  GeneratorConfig,
  LocalConfig,
  JsonGenerator,
  NumberSpecInput,
} from "../types";
import { success, error, validateNumberRange } from "../utils/generator-utils";

/**
 * Generates random numbers within a specified range.
 * Supports both integer and floating-point number generation.
 */
export class NumberSpec implements JsonGenerator<number> {
  constructor(
    public readonly min: number,
    public readonly max: number,
    public readonly integer: boolean = false
  ) {
    validateNumberRange(min, max);
  }

  /**
   * Creates a NumberSpec for integer generation.
   */
  static integer(min: number, max: number): NumberSpec {
    return new NumberSpec(min, max, true);
  }

  /**
   * Creates a NumberSpec for floating-point generation.
   */
  static float(min: number, max: number): NumberSpec {
    return new NumberSpec(min, max, false);
  }

  /**
   * Creates a NumberSpec from a specification object.
   */
  static fromSpec(spec: NumberSpecInput): NumberSpec {
    return new NumberSpec(spec.min, spec.max, spec.integer || false);
  }

  /**
   * Generates a random number within the specified range.
   */
  generate(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<number> {
    try {
      if (this.integer) {
        const value = config.faker.number.int({
          min: Math.ceil(this.min),
          max: Math.floor(this.max),
        });
        return success(value);
      } else {
        const value = config.faker.number.float({
          min: this.min,
          max: this.max,
        });
        return success(value);
      }
    } catch (err) {
      return error(
        `Number generation failed: ${
          err instanceof Error ? err.message : String(err)
        }`
      );
    }
  }

  /**
   * Returns a string representation of this NumberSpec.
   */
  toString(): string {
    const type = this.integer ? "integer" : "float";
    return `NumberSpec(${type}, ${this.min} to ${this.max})`;
  }
}
