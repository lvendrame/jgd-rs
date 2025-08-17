/**
 * Main JGD (JSON Generator Definition) class.
 *
 * This is the primary interface for loading and executing JGD schemas.
 */

import * as fs from "fs";
import * as path from "path";
import type { JsonValue, GenerationResult, CustomKeyFunction } from "../types";
import type { Entity } from "./entity";
import { Jgd as JgdClass } from "./jgd-schema";
import { success, error, validateSchemaMode } from "../utils/generator-utils";
import {
  createConfigWithGlobalKeys,
  addCustomKey as globalAddCustomKey,
} from "../config";

/**
 * Main JGD class for loading and executing JSON generation schemas.
 */
export class Jgd {
  constructor(public readonly schema: JgdClass) {
    // Validate schema mode
    validateSchemaMode(schema.root, schema.entities);
  }

  /**
   * Creates a JGD instance from a schema string.
   */
  static from(schemaString: string): Jgd {
    try {
      const schema = JSON.parse(schemaString) as any;
      return new Jgd(JgdClass.fromSchema(schema));
    } catch (err) {
      throw new Error(
        `Failed to parse JGD schema: ${
          err instanceof Error ? err.message : String(err)
        }`
      );
    }
  }

  /**
   * Creates a JGD instance from a schema file.
   */
  static fromFile(filePath: string): Jgd {
    try {
      const absolutePath = path.resolve(filePath);
      const schemaString = fs.readFileSync(absolutePath, "utf-8");
      return Jgd.from(schemaString);
    } catch (err) {
      throw new Error(
        `Failed to load JGD schema from file '${filePath}': ${
          err instanceof Error ? err.message : String(err)
        }`
      );
    }
  }

  /**
   * Creates a JGD instance from a schema object.
   */
  static fromObject(schema: any): Jgd {
    return new Jgd(JgdClass.fromSchema(schema));
  }

  /**
   * Adds a custom key function to the global registry.
   */
  static addCustomKey(key: string, func: CustomKeyFunction): void {
    globalAddCustomKey(key, func);
  }

  /**
   * Generates JSON data according to the schema definition.
   */
  generate(): GenerationResult<JsonValue> {
    try {
      const config = createConfigWithGlobalKeys(
        this.schema.default_locale || "EN",
        this.schema.seed
      );

      // Handle root mode
      if (this.schema.root) {
        return this.schema.root.generate(config);
      }

      // Handle entities mode
      if (this.schema.entities) {
        const entitiesResult: { [key: string]: JsonValue } = {};

        for (const [entityName, entitySpec] of Object.entries(
          this.schema.entities
        )) {
          const entity = entitySpec as Entity;
          const entityResult = entity.generate(config, { entityName });

          if (!entityResult.success) {
            return error(
              `Entity '${entityName}' generation failed: ${
                (entityResult as any).error
              }`
            );
          }

          entitiesResult[entityName] = entityResult.data;
        }

        return success(entitiesResult);
      }

      // Empty schema
      return success(null);
    } catch (err) {
      return error(
        `Generation failed: ${err instanceof Error ? err.message : String(err)}`
      );
    }
  }

  /**
   * Gets the schema format version.
   */
  getFormat(): string {
    return this.schema.format;
  }

  /**
   * Gets the schema version.
   */
  getVersion(): string {
    return this.schema.version;
  }

  /**
   * Gets the default locale.
   */
  getDefaultLocale(): string {
    return this.schema.default_locale || "EN";
  }

  /**
   * Gets the seed value.
   */
  getSeed(): number | undefined {
    return this.schema.seed;
  }

  /**
   * Checks if the schema is in root mode.
   */
  isRootMode(): boolean {
    return this.schema.root !== undefined;
  }

  /**
   * Checks if the schema is in entities mode.
   */
  isEntitiesMode(): boolean {
    return this.schema.entities !== undefined;
  }

  /**
   * Gets the root entity specification (only in root mode).
   */
  getRootEntity(): Entity | undefined {
    return this.schema.root;
  }

  /**
   * Gets all entity specifications (only in entities mode).
   */
  getEntities(): Record<string, Entity> | undefined {
    return this.schema.entities;
  }

  /**
   * Gets a specific entity specification by name (only in entities mode).
   */
  getEntity(name: string): Entity | undefined {
    return this.schema.entities?.[name];
  }

  /**
   * Returns a string representation of this JGD instance.
   */
  toString(): string {
    const mode = this.isRootMode() ? "root" : "entities";
    const entityCount = this.isEntitiesMode()
      ? Object.keys(this.schema.entities!).length
      : 1;

    return `Jgd(format: ${this.schema.format}, version: ${this.schema.version}, mode: ${mode}, entities: ${entityCount})`;
  }

  /**
   * Validates the schema structure.
   */
  validate(): GenerationResult<boolean> {
    try {
      // Basic format validation
      if (!this.schema.format) {
        return error("Schema missing required $format field");
      }

      if (!this.schema.version) {
        return error("Schema missing required version field");
      }

      // Mode validation
      validateSchemaMode(this.schema.root, this.schema.entities);

      // Validate entities structure
      if (this.schema.entities) {
        for (const [entityName, entitySpec] of Object.entries(
          this.schema.entities
        )) {
          const entity = entitySpec as Entity;
          if (!entity.fields || typeof entity.fields !== "object") {
            return error(
              `Entity '${entityName}' missing or invalid fields specification`
            );
          }
        }
      }

      // Validate root structure
      if (this.schema.root) {
        if (
          !this.schema.root.fields ||
          typeof this.schema.root.fields !== "object"
        ) {
          return error("Root entity missing or invalid fields specification");
        }
      }

      return success(true);
    } catch (err) {
      return error(
        `Schema validation failed: ${
          err instanceof Error ? err.message : String(err)
        }`
      );
    }
  }
}

/**
 * Convenience function to generate JSON data from a schema string.
 */
export function generateFromString(
  schemaString: string
): GenerationResult<JsonValue> {
  try {
    const jgd = Jgd.from(schemaString);
    return jgd.generate();
  } catch (err) {
    return error(
      `Generation from string failed: ${
        err instanceof Error ? err.message : String(err)
      }`
    );
  }
}

/**
 * Convenience function to generate JSON data from a schema file.
 */
export function generateFromFile(
  filePath: string
): GenerationResult<JsonValue> {
  try {
    const jgd = Jgd.fromFile(filePath);
    return jgd.generate();
  } catch (err) {
    return error(
      `Generation from file failed: ${
        err instanceof Error ? err.message : String(err)
      }`
    );
  }
}
