/**
 * Count specification for JGD (JSON Generator Definition) entities.
 *
 * This class represents count specifications for generating multiple items and defines
 * how many items should be generated for arrays, collections, or repeated elements
 * in JGD schemas. It supports both fixed counts and dynamic ranges.
 */

import type { GeneratorConfig } from "../types";

/**
 * Type alias for backward compatibility and cleaner type annotations.
 */
export type CountSpec =
  | number
  | { fixed: number }
  | { range: [number, number] }
  | [number, number];

/**
 * Represents count specifications for generating multiple items.
 *
 * Count defines how many items should be generated for arrays, collections,
 * or repeated elements in JGD schemas. It supports both fixed counts and
 * dynamic ranges, allowing for flexible data generation scenarios.
 *
 * Can be used as:
 * - Direct number: 5
 * - Object form: { fixed: 5 } or { range: [1, 10] }
 * - Array form: [1, 10] (for ranges)
 */
export class Count {
  private readonly spec: CountSpec;

  /**
   * Creates a new Count instance.
   *
   * @param spec - The count specification
   */
  constructor(spec: CountSpec) {
    this.spec = spec;
  }

  /**
   * Creates a fixed count specification.
   *
   * @param value - The exact number of items to generate
   * @returns New Count instance with fixed count
   */
  static fixed(value: number): Count {
    return new Count(value);
  }

  /**
   * Creates a range count specification.
   *
   * @param min - Minimum number of items to generate (inclusive)
   * @param max - Maximum number of items to generate (inclusive)
   * @returns New Count instance with range count
   */
  static range(min: number, max: number): Count {
    return new Count({ range: [min, max] });
  }

  /**
   * Creates a Count instance from various input formats.
   *
   * @param input - Count specification in any supported format
   * @returns New Count instance
   */
  static from(input: CountSpec): Count {
    return new Count(input);
  }

  /**
   * Resolves the count specification to an actual number using the generator configuration.
   *
   * For fixed counts, returns the exact value.
   * For range counts, returns a random number within the range (inclusive).
   *
   * @param config - Generator configuration containing RNG state
   * @returns The resolved count value
   */
  resolve(config: GeneratorConfig): number {
    // Handle direct number (fixed count)
    if (typeof this.spec === "number") {
      return Math.floor(this.spec);
    }

    // Handle array format [min, max] (range count)
    if (Array.isArray(this.spec) && this.spec.length === 2) {
      const [min, max] = this.spec;
      return this.generateRandom(config, min, max);
    }

    // Handle object formats
    if (typeof this.spec === "object" && this.spec !== null) {
      // Handle { fixed: number }
      if ("fixed" in this.spec) {
        return Math.floor(this.spec.fixed);
      }

      // Handle { range: [min, max] }
      if (
        "range" in this.spec &&
        Array.isArray(this.spec.range) &&
        this.spec.range.length === 2
      ) {
        const [min, max] = this.spec.range;
        return this.generateRandom(config, min, max);
      }
    }

    throw new Error(
      `Invalid count specification: ${JSON.stringify(this.spec)}`
    );
  }

  /**
   * Generates a random number within the specified range using the config's RNG.
   *
   * @param config - Generator configuration containing RNG state
   * @param min - Minimum value (inclusive)
   * @param max - Maximum value (inclusive)
   * @returns Random number within range
   */
  private generateRandom(
    config: GeneratorConfig,
    min: number,
    max: number
  ): number {
    const minInt = Math.floor(min);
    const maxInt = Math.floor(max);

    if (minInt > maxInt) {
      throw new Error(`Invalid range: min (${minInt}) > max (${maxInt})`);
    }

    if (minInt === maxInt) {
      return minInt;
    }

    // Use faker's RNG for consistency with the rest of the system
    return config.faker.number.int({ min: minInt, max: maxInt });
  }

  /**
   * Checks if this count specification is fixed (always returns the same value).
   *
   * @returns true if this is a fixed count, false if it's a range
   */
  isFixed(): boolean {
    return (
      typeof this.spec === "number" ||
      (typeof this.spec === "object" &&
        this.spec !== null &&
        "fixed" in this.spec)
    );
  }

  /**
   * Checks if this count specification is a range (returns variable values).
   *
   * @returns true if this is a range count, false if it's fixed
   */
  isRange(): boolean {
    return !this.isFixed();
  }

  /**
   * Gets the fixed value if this is a fixed count.
   *
   * @returns The fixed value, or undefined if this is a range count
   */
  getFixedValue(): number | undefined {
    if (typeof this.spec === "number") {
      return Math.floor(this.spec);
    }

    if (
      typeof this.spec === "object" &&
      this.spec !== null &&
      "fixed" in this.spec
    ) {
      return Math.floor(this.spec.fixed);
    }

    return undefined;
  }

  /**
   * Gets the range values if this is a range count.
   *
   * @returns The [min, max] range, or undefined if this is a fixed count
   */
  getRangeValues(): [number, number] | undefined {
    if (Array.isArray(this.spec) && this.spec.length === 2) {
      return [Math.floor(this.spec[0]), Math.floor(this.spec[1])];
    }

    if (
      typeof this.spec === "object" &&
      this.spec !== null &&
      "range" in this.spec
    ) {
      const [min, max] = this.spec.range;
      return [Math.floor(min), Math.floor(max)];
    }

    return undefined;
  }

  /**
   * Converts this Count to a plain object specification.
   *
   * @returns Plain object representation
   */
  toSpec(): CountSpec {
    return this.spec;
  }

  /**
   * Returns a string representation of this Count.
   */
  toString(): string {
    if (this.isFixed()) {
      const value = this.getFixedValue();
      return `Count.Fixed(${value})`;
    } else {
      const range = this.getRangeValues();
      return `Count.Range([${range![0]}, ${range![1]}])`;
    }
  }
}

/**
 * Utility function to resolve a count specification to an actual number.
 * This function provides backward compatibility with the existing codebase.
 *
 * @param countSpec - Count specification in any supported format
 * @param config - Generator configuration containing RNG state
 * @returns The resolved count value
 */
export function resolveCount(
  countSpec: CountSpec,
  config: GeneratorConfig
): number {
  return Count.from(countSpec).resolve(config);
}
