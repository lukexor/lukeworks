const nextJest = require("next/jest");
const createJestConfig = nextJest({ dir: "./" });

const customJestConfig = {
  testEnvironment: "jsdom",
  moduleDirectories: ["node_modules", "src"],
  testMatch: ["**/__tests__/**/*.[jt]s?(x)", "**/?(*.)+(test).[tj]s?(x)"],
  setupFilesAfterEnv: ["<rootDir>/src/setupTests.ts"],
  clearMocks: true,
  resetMocks: false,
  coverageReporters: ["text", "html"],
};

module.exports = createJestConfig(customJestConfig)();
