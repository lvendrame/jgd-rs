/**
 * JGD (JSON Generator Definition) core schema class.
 *
 * This class represents a complete JGD schema definition and provides generation functionality.
 * JGD is a declarative format for defining JSON data generation rules, supporting both simple
 * root-based schemas and complex multi-entity schemas with cross-references.
 */

import * as fs from "fs";
import * as path from "path";
import type {
  JsonValue,
  GenerationResult,
  GeneratorConfig,
  CustomKeyFunction,
} from "../types";
import { Entity } from "./entity";
import { success, error, validateSchemaMode } from "../utils/generator-utils";
import {
  createConfigWithGlobalKeys,
  addCustomKey as globalAddCustomKey,
} from "../config";

/**
 * Core JGD schema representation containing all schema metadata and entity definitions.
 *
 * The Jgd class represents a complete JSON Generator Definition schema that can be loaded
 * from JSON objects or strings. It supports two mutually exclusive generation modes:
 * - Root mode: Single entity definition in the root field
 * - Entities mode: Multiple named entities in the entities collection
 */
export class Jgd {
  /**
   * Schema format identifier (e.g., "jgd/v1").
   *
   * This field identifies the JGD format version and is used for schema validation
   * and compatibility checking.
   */
  public format: string;

  /**
   * User-defined schema version string.
   *
   * This field allows schema authors to version their JGD definitions
   * for tracking changes and compatibility management.
   */
  public version: string;

  /**
   * Optional random seed for deterministic generation.
   *
   * When provided, this seed ensures reproducible data generation across multiple
   * executions. When undefined, generation uses non-deterministic randomness.
   */
  public seed?: number;

  /**
   * Default locale for fake data generation.
   *
   * Specifies the locale code (e.g., "EN", "FR", "DE") used for generating
   * locale-specific fake data. Defaults to "EN" when not specified.
   */
  public default_locale: string = "EN";

  /**
   * Named entity definitions for entities mode (mutually exclusive with root).
   *
   * When present, the schema operates in entities mode where multiple named
   * entities are generated. Each key represents an entity name, and the value
   * contains the entity definition with its fields and generation rules.
   */
  public entities?: Record<string, Entity>;

  /**
   * Root entity definition for root mode (mutually exclusive with entities).
   *
   * When present, the schema operates in root mode where a single entity
   * structure is generated. The entity definition contains fields and
   * generation rules applied to the root level.
   */
  public root?: Entity;

  /**
   * Creates a new Jgd instance.
   *
   * @param format - Schema format identifier
   * @param version - User-defined schema version
   * @param options - Optional configuration
   */
  constructor(
    format: string,
    version: string,
    options: {
      seed?: number;
      default_locale?: string;
      entities?: Record<string, Entity>;
      root?: Entity;
    } = {}
  ) {
    this.format = format;
    this.version = version;
    this.seed = options.seed;
    this.default_locale = options.default_locale || "EN";
    this.entities = options.entities;
    this.root = options.root;

    // Validate schema structure during construction
    this.validateStructure();
  }

  /**
   * Creates a Jgd instance from a plain object schema.
   *
   * @param schema - Plain object containing JGD schema
   * @returns New Jgd instance
   */
  static fromSchema(schema: {
    $format: string;
    version: string;
    seed?: number;
    defaultLocale?: string;
    default_locale?: string; // Support both naming conventions
    entities?: Record<string, any>;
    root?: any;
  }): Jgd {
    // Convert plain object entities to Entity instances
    const entities = schema.entities
      ? Object.fromEntries(
          Object.entries(schema.entities).map(([name, spec]) => [
            name,
            Entity.fromSpec(spec as any),
          ])
        )
      : undefined;

    // Convert root spec to Entity instance
    const root = schema.root ? Entity.fromSpec(schema.root as any) : undefined;

    // Support both defaultLocale and default_locale naming
    const default_locale =
      schema.default_locale || schema.defaultLocale || "EN";

    return new Jgd(schema.$format, schema.version, {
      seed: schema.seed,
      default_locale,
      entities,
      root,
    });
  }

  /**
   * Creates a Jgd instance from a JSON string.
   *
   * @param jsonString - JSON string containing JGD schema
   * @returns New Jgd instance
   */
  static fromString(jsonString: string): Jgd {
    try {
      const parsed = JSON.parse(jsonString);
      return Jgd.fromSchema(parsed);
    } catch (error) {
      throw new Error(
        `Failed to parse JGD schema: ${
          error instanceof Error ? error.message : String(error)
        }`
      );
    }
  }

  /**
   * Converts this Jgd to a plain object schema.
   *
   * @returns Plain object representation compatible with JSON serialization
   */
  toSchema(): {
    $format: string;
    version: string;
    seed?: number;
    defaultLocale: string;
    entities?: Record<string, any>;
    root?: any;
  } {
    const schema: any = {
      $format: this.format,
      version: this.version,
      defaultLocale: this.default_locale,
    };

    if (this.seed !== undefined) {
      schema.seed = this.seed;
    }

    if (this.entities) {
      schema.entities = Object.fromEntries(
        Object.entries(this.entities).map(([name, entity]) => [
          name,
          entity.toSpec(),
        ])
      );
    }

    if (this.root) {
      schema.root = this.root.toSpec();
    }

    return schema;
  }

  /**
   * Converts this Jgd to a JSON string.
   *
   * @param space - Optional spacing for pretty-printing
   * @returns JSON string representation
   */
  toJSON(space?: string | number): string {
    return JSON.stringify(this.toSchema(), null, space);
  }

  /**
   * Checks if this schema is in root mode.
   *
   * @returns true if schema has root entity defined
   */
  isRootMode(): boolean {
    return this.root !== undefined;
  }

  /**
   * Checks if this schema is in entities mode.
   *
   * @returns true if schema has entities defined
   */
  isEntitiesMode(): boolean {
    return this.entities !== undefined;
  }

  /**
   * Validates that the schema has either root or entities (but not both).
   * This method throws errors to maintain backward compatibility.
   *
   * @throws Error if schema is invalid
   */
  private validateStructure(): void {
    const hasRoot = this.root !== undefined;
    const hasEntities = this.entities !== undefined;

    if (!hasRoot && !hasEntities) {
      throw new Error(
        "JGD schema must have either 'root' or 'entities' defined"
      );
    }

    if (hasRoot && hasEntities) {
      throw new Error(
        "JGD schema cannot have both 'root' and 'entities' defined at the same time"
      );
    }
  }

  /**
   * Validates the schema structure (shorter alias for validateSchema).
   */
  validate(): GenerationResult<boolean> {
    return this.validateSchema();
  }
  /**
   * Returns a string representation of this Jgd.
   */
  toString(): string {
    const mode = this.isRootMode() ? "root" : "entities";
    const entityCount = this.entities ? Object.keys(this.entities).length : 0;
    return `Jgd(format: ${this.format}, version: ${this.version}, mode: ${mode}, entities: ${entityCount})`;
  }

  // ========== Generation and Utility Methods (merged from jgd.ts) ==========

  /**
   * Creates a JGD instance from a schema string.
   */
  static from(schemaString: string): Jgd {
    try {
      const schema = JSON.parse(schemaString) as any;
      return Jgd.fromSchema(schema);
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
    return Jgd.fromSchema(schema);
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
        this.default_locale || "EN",
        this.seed
      );

      // Handle root mode
      if (this.root) {
        return this.root.generate(config);
      }

      // Handle entities mode
      if (this.entities) {
        const entitiesResult: { [key: string]: JsonValue } = {};

        for (const [entityName, entitySpec] of Object.entries(this.entities)) {
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
    return this.format;
  }

  /**
   * Gets the schema version.
   */
  getVersion(): string {
    return this.version;
  }

  /**
   * Gets the default locale.
   */
  getDefaultLocale(): string {
    return this.default_locale || "EN";
  }

  /**
   * Gets the seed value.
   */
  getSeed(): number | undefined {
    return this.seed;
  }

  /**
   * Gets the root entity specification (only in root mode).
   */
  getRootEntity(): Entity | undefined {
    return this.root;
  }

  /**
   * Gets all entity specifications (only in entities mode).
   */
  getEntities(): Record<string, Entity> | undefined {
    return this.entities;
  }

  /**
   * Gets a specific entity specification by name (only in entities mode).
   */
  getEntity(name: string): Entity | undefined {
    return this.entities?.[name];
  }

  /**
   * Validates the schema structure.
   */
  validateSchema(): GenerationResult<boolean> {
    try {
      // Basic format validation
      if (!this.format) {
        return error("Schema missing required $format field");
      }

      if (!this.version) {
        return error("Schema missing required version field");
      }

      // Mode validation
      validateSchemaMode(this.root, this.entities);

      // Validate entities structure
      if (this.entities) {
        for (const [entityName, entitySpec] of Object.entries(this.entities)) {
          const entity = entitySpec as Entity;
          if (!entity.fields || typeof entity.fields !== "object") {
            return error(
              `Entity '${entityName}' missing or invalid fields specification`
            );
          }
        }
      }

      // Validate root structure
      if (this.root) {
        if (!this.root.fields || typeof this.root.fields !== "object") {
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
