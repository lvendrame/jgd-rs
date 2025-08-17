const { Jgd } = require("./dist");

const schema = {
  $format: "jgd/v1",
  version: "1.0.0",
  root: {
    count: 2,
    fields: {
      id: "${ulid}",
      name: "${name.firstName}",
      email: "${internet.safeEmail}",
    },
  },
};

console.log("Testing merged Entity class...");
const jgd = Jgd.fromObject(schema);
console.log("✓ JGD created successfully");

const result = jgd.generate();
if (result.success) {
  console.log("✓ Generation successful");
  console.log("Generated data:", JSON.stringify(result.data, null, 2));
} else {
  console.log("✗ Generation failed:", result.error);
}
