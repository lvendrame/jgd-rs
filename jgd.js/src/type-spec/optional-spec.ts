/**
 * Optional specification wrapper that makes any field optionally null.
 */

import type {
  JsonValue,
  GenerationResult,
  GeneratorConfig,
  LocalConfig,
  JsonGenerator,
} from "../types";
import { success } from "../utils/generator-utils";

/**
 * Input specification for optional fields.
 */
export interface OptionalSpecInput<T = unknown> {
  optional: {
    prob?: number; // 0.0 to 1.0, defaults to 0.5
    of: T;
  };
}

// Import FieldSpec from field.ts to avoid circular dependency
import type { FieldSpec } from "./field";

/**
 * Wraps any field specification to make it optionally null based on probability.
 */
export class OptionalSpec implements JsonGenerator<JsonValue> {
  constructor(
    public readonly innerSpec: FieldSpec,
    public readonly probability: number = 0.5
  ) {
    if (probability < 0 || probability > 1) {
      throw new Error(
        `Probability must be between 0 and 1, got: ${probability}`
      );
    }
  }

  /**
   * Creates an OptionalSpec from a specification object.
   */
  static fromSpec<T>(spec: OptionalSpecInput<T>): OptionalSpec {
    const probability = spec.optional.prob ?? 0.5;
    return new OptionalSpec(spec.optional.of as FieldSpec, probability);
  }

  /**
   * Generates either null or the wrapped value based on probability.
   */
  generate(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue> {
    // Generate random number to determine if we should return null
    const random = config.faker.number.float({ min: 0, max: 1 });

    if (random >= this.probability) {
      return success(null);
    }

    // Generate the actual value using FieldGenerator
    // Import here to avoid circular dependency
    const { FieldGenerator } = require("./field");
    const fieldGenerator = new FieldGenerator(this.innerSpec);
    return fieldGenerator.generate(config, localConfig);
  }

  /**
   * Returns a string representation of this OptionalSpec.
   */
  toString(): string {
    return `OptionalSpec(probability: ${
      this.probability
    }, spec: ${JSON.stringify(this.innerSpec)})`;
  }
}
