/**
 * Field generator that handles all types of field specifications.
 */

import type {
  JsonValue,
  GenerationResult,
  GeneratorConfig,
  LocalConfig,
  JsonGenerator,
  FieldSpec,
  NumberSpecInput,
  ArraySpecInput,
  OptionalSpecInput,
  EntitySpec,
} from "../types";
import { success, error, pushDepth, popDepth } from "../utils/generator-utils";
import { NumberSpec } from "./number-spec";
import { ArraySpec } from "./array-spec";
import { Replacer } from "../utils/replacer";
import { processTemplate, isTemplate } from "../template";

/**
 * Generates values for any type of field specification.
 * Acts as a dispatcher to the appropriate generator based on the field type.
 */
export class FieldGenerator implements JsonGenerator<JsonValue> {
  constructor(public readonly fieldSpec: FieldSpec) {}

  /**
   * Generates a value based on the field specification type.
   */
  generate(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue> {
    // Handle primitive values
    if (typeof this.fieldSpec === "string") {
      return this.handleStringField(config, localConfig);
    }

    if (
      typeof this.fieldSpec === "number" ||
      typeof this.fieldSpec === "boolean"
    ) {
      return success(this.fieldSpec);
    }

    // Handle object specifications
    if (typeof this.fieldSpec === "object" && this.fieldSpec !== null) {
      return this.handleObjectField(config, localConfig);
    }

    return error(
      `Unsupported field specification: ${JSON.stringify(this.fieldSpec)}`
    );
  }

  /**
   * Handles string field specifications (literals or faker patterns).
   */
  private handleStringField(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue> {
    const stringSpec = this.fieldSpec as string;

    // Check if it contains any template patterns
    if (isTemplate(stringSpec)) {
      return processTemplate(stringSpec, config, localConfig);
    }

    // Return as literal string
    return success(stringSpec);
  }

  /**
   * Handles object field specifications (NumberSpec, ArraySpec, etc.).
   */
  private handleObjectField(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue> {
    const spec = this.fieldSpec as Record<string, any>;

    // Handle NumberSpec
    if ("min" in spec && "max" in spec) {
      const numberSpec = NumberSpec.fromSpec(spec as NumberSpecInput);
      return numberSpec.generate(config, localConfig);
    }

    // Handle number wrapper: { number: NumberSpec }
    if ("number" in spec) {
      const numberSpec = NumberSpec.fromSpec(spec.number as NumberSpecInput);
      return numberSpec.generate(config, localConfig);
    }

    // Handle ArraySpec (direct array specification)
    if ("count" in spec && "of" in spec) {
      const arraySpec = ArraySpec.fromSpec(spec as ArraySpecInput);
      return arraySpec.generate(config, localConfig);
    }

    // Handle array wrapper: { array: ArraySpec }
    if ("array" in spec) {
      const arraySpec = ArraySpec.fromSpec(spec.array as ArraySpecInput);
      return arraySpec.generate(config, localConfig);
    }

    // Handle OptionalSpec
    if ("optional" in spec) {
      const { OptionalSpec } = require("./optional-spec");
      const optionalSpec = OptionalSpec.fromSpec(spec as OptionalSpecInput);
      return optionalSpec.generate(config, localConfig);
    }

    // Handle reference: { ref: "entity.field" }
    if ("ref" in spec) {
      return this.handleReference(spec.ref, config, localConfig);
    }

    // Handle EntitySpec (nested object with fields)
    if ("fields" in spec) {
      const { EntityGenerator } = require("./entity");
      const entityGenerator = new EntityGenerator(spec as EntitySpec);
      return entityGenerator.generate(config, localConfig);
    }

    return error(`Unknown object field specification: ${JSON.stringify(spec)}`);
  }

  /**
   * Handles reference fields that point to other entity values.
   */
  private handleReference(
    ref: string,
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue> {
    // For now, return a placeholder - full reference resolution would require
    // access to generated entities data
    return success(`ref:${ref}`);
  }

  /**
   * Returns a string representation of this FieldGenerator.
   */
  toString(): string {
    return `FieldGenerator(${JSON.stringify(this.fieldSpec)})`;
  }
}
