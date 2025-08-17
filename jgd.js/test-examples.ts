#!/usr/bin/env node

/**
 * JGD Examples Test - Tests all Rust examples with TypeScript implementation
 */

import { Jgd, addCustomKey } from "./src/index";
import { readFileSync, readdirSync } from "fs";
import { join } from "path";

// Add a custom key for testing custom key examples
addCustomKey("custom.key", (args) => {
  switch (args.type) {
    case "none":
      return { success: true, data: "default-custom-value" };
    case "fixed":
      return { success: true, data: `custom-${args.value}` };
    case "range":
      return { success: true, data: `custom-${args.min}-${args.max}` };
  }
});

console.log("🧪 JGD Examples Test\n");

const examplesDir = "./examples";
const examples = readdirSync(examplesDir)
  .filter((file) => file.endsWith(".jgd"))
  .sort();

let totalTests = 0;
let passedTests = 0;

for (const example of examples) {
  totalTests++;
  const filePath = join(examplesDir, example);

  console.log(`📄 Testing: ${example}`);

  try {
    // Load and generate
    const jgd = Jgd.fromFile(filePath);
    const result = jgd.generate();

    if (result.success) {
      passedTests++;
      console.log(`✅ PASS - Generated successfully`);

      // Show a preview of the generated data
      const preview = JSON.stringify(result.data, null, 2);
      const previewLines = preview.split("\n").slice(0, 10);
      if (preview.split("\n").length > 10) {
        previewLines.push("  ... (truncated)");
      }
      console.log("   Preview:");
      previewLines.forEach((line) => console.log(`   ${line}`));
    } else {
      console.log(`❌ FAIL - Generation error: ${result.error}`);
    }
  } catch (error) {
    console.log(
      `❌ FAIL - Exception: ${
        error instanceof Error ? error.message : String(error)
      }`
    );
  }

  console.log("");
}

console.log(`📊 Results: ${passedTests}/${totalTests} tests passed`);

if (passedTests === totalTests) {
  console.log(
    "🎉 All Rust examples work perfectly with TypeScript implementation!"
  );
  process.exit(0);
} else {
  console.log("⚠️  Some examples failed - check compatibility issues");
  process.exit(1);
}
