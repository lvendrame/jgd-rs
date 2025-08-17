/**
 * JGD (JSON Generator Definition) core schema class.
 *
 * This class mirrors the Rust Jgd struct and represents a complete JGD schema definition.
 * JGD is a declarative format for defining JSON data generation rules, supporting both simple
 * root-based schemas and complex multi-entity schemas with cross-references.
 */

import type { JsonValue, GenerationResult, GeneratorConfig } from "../types";
import { Entity } from "./entity";

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
   *
   * @throws Error if schema is invalid
   */
  validate(): void {
    const hasRoot = this.root !== undefined;
    const hasEntities = this.entities !== undefined;

    if (!hasRoot && !hasEntities) {
      throw new Error(
        "JGD schema must have either 'root' or 'entities' defined"
      );
    }

    if (hasRoot && hasEntities) {
      throw new Error(
        "JGD schema cannot have both 'root' and 'entities' defined"
      );
    }
  }

  /**
   * Returns a string representation of this Jgd.
   */
  toString(): string {
    const mode = this.isRootMode() ? "root" : "entities";
    const entityCount = this.entities ? Object.keys(this.entities).length : 0;
    return `Jgd(format: ${this.format}, version: ${this.version}, mode: ${mode}, entities: ${entityCount})`;
  }
}
