#!/usr/bin/env node

/**
 * Demo script showcasing JGD.js functionality
 */

import { Jgd, addCustomKey, isSuccess } from "./src/index";

console.log("🚀 JGD.js Demo\n");

// Add a custom key
addCustomKey("custom.status", (args) => {
  const statuses = ["active", "inactive", "pending", "suspended"];
  const index = Math.floor(Math.random() * statuses.length);
  return { success: true, data: statuses[index] };
});

// Demo 1: Simple Root Entity
console.log("📄 Demo 1: Simple Root Entity");
const simpleSchema = {
  $format: "jgd/v1",
  version: "1.0.0",
  seed: 42,
  root: {
    fields: {
      name: "${person.firstName}",
      age: { min: 18, max: 65, integer: true },
      email: "${internet.email}",
      status: "${custom.status}",
      tags: {
        array: {
          count: 3,
          of: "${lorem.word}",
        },
      },
    },
  },
};

const jgd1 = Jgd.fromObject(simpleSchema);
const result1 = jgd1.generate();

if (isSuccess(result1)) {
  console.log(JSON.stringify(result1.data, null, 2));
} else {
  console.error("Error:", result1.error);
}

console.log("\n---\n");

// Demo 2: Multiple Users with Posts
console.log("👥 Demo 2: Multiple Users with Posts");
const usersSchema = {
  $format: "jgd/v1",
  version: "1.0.0",
  seed: 123,
  entities: {
    users: {
      count: 2,
      fields: {
        id: "${index}",
        name: "${person.fullName}",
        email: "${internet.email}",
        summary: "User ${index} of ${count}",
        posts: {
          count: { range: [1, 2] as [number, number] },
          fields: {
            id: "${string.uuid}",
            userId: "${index(2)}", // Reference parent user
            title: "${lorem.sentence}",
            content: "${lorem.paragraph}",
            postNumber: "Post ${index} by user ${index(2)}",
          },
        },
      },
    },
    companies: {
      count: 1,
      fields: {
        id: "${index}",
        name: "${company.name}",
        employees: {
          array: {
            count: 3,
            of: "${person.firstName}",
          },
        },
      },
    },
  },
};

const jgd2 = Jgd.fromObject(usersSchema);
const result2 = jgd2.generate();

if (isSuccess(result2)) {
  console.log(JSON.stringify(result2.data, null, 2));
} else {
  console.error("Error:", result2.error);
}

console.log("\n---\n");

// Demo 3: Number and Array Specifications
console.log("🔢 Demo 3: Number and Array Specifications");
const numberSchema = {
  $format: "jgd/v1",
  version: "1.0.0",
  seed: 456,
  root: {
    count: 2,
    fields: {
      id: "${index}",
      score: { min: 0, max: 100, integer: true },
      rating: { min: 1.0, max: 5.0 },
      metadata: {
        fields: {
          tags: {
            array: {
              count: { range: [2, 4] as [number, number] },
              of: "${lorem.word}",
            },
          },
          scores: {
            array: {
              count: 3,
              of: { min: 10, max: 20, integer: true },
            },
          },
        },
      },
    },
  },
};

const jgd3 = Jgd.fromObject(numberSchema);
const result3 = jgd3.generate();

if (isSuccess(result3)) {
  console.log(JSON.stringify(result3.data, null, 2));
} else {
  console.error("Error:", result3.error);
}

console.log("\n✨ Demo completed!");
console.log("\nKey Features Demonstrated:");
console.log("• Faker integration (${person.firstName}, ${lorem.word}, etc.)");
console.log("• Custom key functions (${custom.status})");
console.log("• Context-aware keys (${index}, ${count}, ${index(2)})");
console.log("• Number specifications with ranges");
console.log("• Array generation for primitives");
console.log("• Nested entity structures");
console.log("• Deterministic generation with seeds");
console.log("• Both root and entities modes");
