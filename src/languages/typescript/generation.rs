pub fn generate_typescript_config_files() -> (String, String, String) {
    let package_json = r#"{
    "name": "kata",
    "version": "1.0.0",
    "scripts": {
        "test": "jest"
    },
    "devDependencies": {
        "@jest/globals": "^29.5.0",
        "jest": "^29.5.0",
        "ts-jest": "^29.1.0",
        "typescript": "^5.0.4"
    }
}"#
    .trim()
    .to_string();

    let jest_config = r#"module.exports = {
    transform: { '^.+\\.ts?$': 'ts-jest' },
    testEnvironment: 'node',
    testRegex: '[a-z]+.spec.ts',
    moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx', 'json', 'node']
};"#
    .trim()
    .to_string();

    let ts_config = r#"{
    "compilerOptions": {
      "target": "es2016",
      "module": "commonjs",
      "esModuleInterop": true,
      "forceConsistentCasingInFileNames": true,
      "strict": true,
      "skipLibCheck": true
    }
}"#
    .trim()
    .to_string();

    (package_json, jest_config, ts_config)
}

pub fn generate_typescript_calculator_files() -> (String, String) {
    let program_file_content = r#"export function addNumbers(num1: string, num2: string): number {
    // Implement the addition logic here
    // ...
}"#
    .trim()
    .to_string();

    let test_file_content = r#"import { addNumbers } from './calculator';
import { describe, it, expect } from '@jest/globals';

describe('Calculator', () => {
    it('should add numbers correctly', () => {
        expect(addNumbers("1", "2")).toEqual(3);
        // Add more test cases here
    });
});"#
        .trim()
        .to_string();

    (program_file_content, test_file_content)
}

pub fn generate_typescript_dsa_files() -> (String, String) {
    let program_file_content = r#"
        // TODO
    "#
    .to_string();

    let test_file_content = r#"
        // TODO
    "#
    .to_string();

    (program_file_content, test_file_content)
}
