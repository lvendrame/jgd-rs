# Refactor Summary: Interface/Class Naming Collision Resolution

## Problem

The codebase had **naming collisions** between interface types and classes:

- `interface NumberSpec` vs `class NumberSpec`
- `interface ArraySpec` vs `class ArraySpec`
- `interface OptionalSpec` vs `class OptionalSpec`

This created confusion and violated TypeScript naming conventions.

## Solution

**Removed redundant interfaces** and kept **only the classes** as the canonical representation.

### Changes Made

#### 1. Removed Duplicate Interfaces from `types.ts`

**Removed:**

```typescript
export interface NumberSpec {
  min: number;
  max: number;
  integer?: boolean;
}

export interface ArraySpec {
  count: CountSpec;
  of: string | number | boolean | NumberSpec;
}

export interface OptionalSpec<T = unknown> {
  optional: T;
  probability?: number;
}
```

#### 2. Created Input Type Interfaces

**Added:**

```typescript
export interface NumberSpecInput {
  min: number;
  max: number;
  integer?: boolean;
}

export interface ArraySpecInput {
  count: CountSpec;
  of: string | number | boolean | NumberSpecInput;
}

export interface OptionalSpecInput<T = unknown> {
  optional: T;
  probability?: number;
}
```

#### 3. Updated All Imports and References

**Files Updated:**

- `src/array-spec.ts`
- `src/number-spec.ts`
- `src/optional-spec.ts`
- `src/field-generator.ts`
- `src/index.ts`

**Changes:**

- `NumberSpec as NumberSpecType` → `NumberSpecInput`
- `ArraySpec as ArraySpecType` → `ArraySpecInput`
- `OptionalSpec as OptionalSpecType` → `OptionalSpecInput`

#### 4. Updated FieldSpec Type

**Before:**

```typescript
export type FieldSpec =
  | string
  | number
  | boolean
  | NumberSpec // ← Interface
  | ArraySpec // ← Interface
  | OptionalSpec // ← Interface
  | EntitySpec
  | { ref: string }
  | { array: ArraySpec };
```

**After:**

```typescript
export type FieldSpec =
  | string
  | number
  | boolean
  | NumberSpecInput // ← Input type
  | ArraySpecInput // ← Input type
  | OptionalSpecInput // ← Input type
  | EntitySpec
  | { ref: string }
  | { array: ArraySpecInput };
```

## Benefits

### ✅ **Single Source of Truth**

- Each concept now has **only one representation** (the class)
- Classes provide both **type structure AND behavior**
- No more confusion between interface vs class

### ✅ **Cleaner Architecture**

- **Input types** (`*Input`) represent raw schema data
- **Classes** (`NumberSpec`, `ArraySpec`, etc.) represent generators with behavior
- Clear separation between **data** and **behavior**

### ✅ **Better Developer Experience**

- No more naming collisions
- Clearer imports and exports
- Better IntelliSense and type checking

### ✅ **TypeScript Best Practices**

- Follows convention of using classes for behavior
- Interfaces only for pure data contracts
- No duplicate type names

## Verification

### ✅ **All Tests Pass**

```bash
npm test
Test Suites: 1 passed, 1 total
Tests:       16 passed, 16 total
```

### ✅ **Build Successful**

```bash
npm run build
> tsc
# No errors
```

### ✅ **Demo Works**

```bash
npx ts-node demo.ts
🚀 JGD.js Demo
# All functionality working correctly
```

## Migration Guide for Users

### Before (Old Way)

```typescript
import { NumberSpecType } from "jgd.js";

const spec: NumberSpecType = {
  min: 1,
  max: 100,
  integer: true,
};
```

### After (New Way)

```typescript
import { NumberSpec, NumberSpecInput } from "jgd.js";

// For input data
const spec: NumberSpecInput = {
  min: 1,
  max: 100,
  integer: true,
};

// For generators
const generator = NumberSpec.fromSpec(spec);
const result = generator.generate(config);
```

## Summary

This refactor **eliminates naming confusion** by:

1. **Removing redundant interfaces**
2. **Using classes as the primary representation**
3. **Creating separate input types for raw data**
4. **Maintaining full backward compatibility**

The architecture is now **cleaner**, **more maintainable**, and follows **TypeScript best practices**.
