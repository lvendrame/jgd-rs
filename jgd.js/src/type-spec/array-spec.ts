/**
 * Array specification generator for creating arrays of primitive values.
 */

import type {
  JsonValue,
  GenerationResult,
  GeneratorConfig,
  LocalConfig,
  JsonGenerator,
  ArraySpecInput,
  NumberSpecInput,
} from "../types";
import {
  success,
  error,
  resolveCount,
  pushDepth,
  popDepth,
} from "../utils/generator-utils";
import { NumberSpec } from "./number-spec";
import { Replacer } from "../utils/replacer";
import { processTemplate, isTemplate } from "../template";

/**
 * Generates arrays of primitive values (strings, numbers, booleans).
 * Arrays in JGD are strictly for primitive types - complex objects should use entities.
 */
export class ArraySpec implements JsonGenerator<JsonValue[]> {
  constructor(
    public readonly count: ArraySpecInput["count"],
    public readonly elementSpec:
      | string
      | number
      | boolean
      | NumberSpecInput
      | { ref: string }
  ) {}

  /**
   * Creates an ArraySpec from a specification object.
   */
  static fromSpec(spec: ArraySpecInput): ArraySpec {
    return new ArraySpec(spec.count, spec.of);
  }

  /**
   * Generates an array of primitive values.
   */
  generate(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue[]> {
    try {
      const itemCount = resolveCount(this.count, config);
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

    return `ArraySpec(count: ${JSON.stringify(
      this.count
    )}, element: ${elementType})`;
  }
}
