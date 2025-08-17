/**
 * Entity generator for creating complex objects with multiple fields.
 */

import type {
  JsonValue,
  GenerationResult,
  GeneratorConfig,
  LocalConfig,
  JsonGenerator,
} from "../types";
import type { EntitySpec } from "./entity-spec";
import {
  success,
  error,
  resolveCount,
  pushDepth,
  popDepth,
} from "../utils/generator-utils";
import { FieldGenerator } from "./field";

/**
 * Generates complex objects (entities) with multiple fields.
 * Can generate single objects or arrays of objects based on count specification.
 */
export class EntityGenerator implements JsonGenerator<JsonValue> {
  constructor(public readonly entitySpec: EntitySpec) {}

  /**
   * Generates an entity or array of entities based on count specification.
   */
  generate(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue> {
    try {
      // If no count is specified, generate a single object
      if (!this.entitySpec.count) {
        return this.generateSingleEntity(config, localConfig);
      }

      // Generate multiple entities
      const itemCount = resolveCount(this.entitySpec.count, config);
      const entities: JsonValue[] = [];

      for (let i = 0; i < itemCount; i++) {
        // Push depth for entity iteration
        pushDepth(
          config,
          i + 1,
          itemCount,
          localConfig?.entityName,
          localConfig?.fieldName
        );

        try {
          const entityResult = this.generateSingleEntity(config, {
            ...localConfig,
            index: i + 1,
            count: itemCount,
          });

          if (!entityResult.success) {
            popDepth(
              config,
              !!localConfig?.entityName,
              !!localConfig?.fieldName
            );
            return error(
              `Entity generation failed: ${(entityResult as any).error}`
            );
          }

          entities.push(entityResult.data);
        } finally {
          popDepth(config, !!localConfig?.entityName, !!localConfig?.fieldName);
        }
      }

      return success(entities);
    } catch (err) {
      return error(
        `Entity generation failed: ${
          err instanceof Error ? err.message : String(err)
        }`
      );
    }
  }

  /**
   * Generates a single entity object with all its fields.
   */
  private generateSingleEntity(
    config: GeneratorConfig,
    localConfig?: LocalConfig
  ): GenerationResult<JsonValue> {
    const entityObject: { [key: string]: JsonValue } = {};

    // Generate each field
    for (const [fieldName, fieldSpec] of Object.entries(
      this.entitySpec.fields
    )) {
      // Update field name in config stack
      config.fieldNameStack.push(fieldName);

      try {
        const fieldGenerator = new FieldGenerator(fieldSpec);
        const fieldResult = fieldGenerator.generate(config, {
          ...localConfig,
          fieldName,
        });

        if (!fieldResult.success) {
          config.fieldNameStack.pop();
          return error(
            `Field '${fieldName}' generation failed: ${
              (fieldResult as any).error
            }`
          );
        }

        entityObject[fieldName] = fieldResult.data;
      } finally {
        config.fieldNameStack.pop();
      }
    }

    return success(entityObject);
  }

  /**
   * Returns a string representation of this EntityGenerator.
   */
  toString(): string {
    const fieldCount = Object.keys(this.entitySpec.fields).length;
    const hasCount = this.entitySpec.count !== undefined;
    return `EntityGenerator(fields: ${fieldCount}, hasCount: ${hasCount})`;
  }
}
