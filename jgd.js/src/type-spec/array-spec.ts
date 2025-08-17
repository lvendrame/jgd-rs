/**
 * Array specification generator for creating arrays of primitive values.
 */

import type {
  JsonValue,
  GenerationResult,
  GeneratorConfig,
  LocalConfig,
  JsonGenerator,
} from "../types";
import { Count } from "./count";
import { success, error, pushDepth, popDepth } from "../utils/generator-utils";
import { NumberSpec } from "./number-spec";
import { processTemplate, isTemplate } from "../template";

/**
 * Generates arrays of primitive values (strings, numbers, booleans).
 * Arrays in JGD are strictly for primitive types - complex objects should use entities.
 *
 * Can be used directly as a class or created from plain object specifications.
 */
export class ArraySpec implements JsonGenerator<JsonValue[]> {
  constructor(
    public readonly count: Count,
    public readonly elementSpec:
      | string
      | number
      | boolean
      | { min: number; max: number; integer?: boolean }
      | { ref: string }
  ) {}

  /**
   * Creates an ArraySpec from a specification object.
   */
  static fromSpec(spec: {
    count:
      | Count
      | number
      | { fixed: number }
      | { range: [number, number] }
      | [number, number];
    of:
      | string
      | number
      | boolean
      | { min: number; max: number; integer?: boolean }
      | { ref: string };
  }): ArraySpec {
    const count =
      spec.count instanceof Count ? spec.count : Count.from(spec.count);
    return new ArraySpec(count, spec.of);
  }

  /**
   * Generates an array of primitive values.
   */
  generate(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue[]> {
    try {
      const itemCount = this.count.resolve(config);
      const result: JsonValue[] = [];

      for (let i = 0; i < itemCount; i++) {
        // Push depth for array iteration
        pushDepth(config, i + 1, itemCount);

        try {
          const elementResult = this.generateElement(config, localConfig);
          if (!elementResult.success) {
            popDepth(config);
            return error(
              `Array element generation failed: ${(elementResult as any).error}`
            );
          }

          result.push(elementResult.data);
        } finally {
          popDepth(config);
        }
      }

      return success(result);
    } catch (err) {
      return error(
        `Array generation failed: ${
          err instanceof Error ? err.message : String(err)
        }`
      );
    }
  }

  /**
   * Generates a single element based on the element specification.
   */
  private generateElement(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue> {
    // Handle primitive values
    if (typeof this.elementSpec === "string") {
      // Check if it contains any template patterns
      if (isTemplate(this.elementSpec)) {
        return processTemplate(this.elementSpec, config, localConfig);
      }
      return success(this.elementSpec);
    }

    if (
      typeof this.elementSpec === "number" ||
      typeof this.elementSpec === "boolean"
    ) {
      return success(this.elementSpec);
    }

    // Handle NumberSpec
    if (
      typeof this.elementSpec === "object" &&
      "min" in this.elementSpec &&
      "max" in this.elementSpec
    ) {
      const numberSpec = NumberSpec.fromSpec(this.elementSpec);
      return numberSpec.generate(config, localConfig);
    }

    // Handle reference objects: { ref: "entity.field" }
    if (
      typeof this.elementSpec === "object" &&
      this.elementSpec !== null &&
      "ref" in this.elementSpec
    ) {
      // For now, return a placeholder - full reference resolution would require
      // access to generated entities data
      return success(`ref:${(this.elementSpec as { ref: string }).ref}`);
    }

    return error(`Unsupported array element type: ${typeof this.elementSpec}`);
  }

  /**
   * Returns a string representation of this ArraySpec.
   */
  toString(): string {
    const elementType =
      typeof this.elementSpec === "object"
        ? JSON.stringify(this.elementSpec)
        : String(this.elementSpec);

    return `ArraySpec(count: ${this.count.toString()}, element: ${elementType})`;
  }
}
