pub fn generate_csharp_calculator_files() -> (String, String) {
    let program_file_content = r#"public static class Calculator {
    public static int AddNumbers(string num1, string num2) {
        // Implement the addition logic here
        // ...
    }
}"#
    .trim()
    .to_string();

    let test_file_content = r#"using NUnit.Framework;

[TestFixture]
public class CalculatorTests {
    [Test]
    public void TestAddNumbers() {
        Assert.AreEqual(5, Calculator.AddNumbers("2", "3"));
        Assert.AreEqual(5, Calculator.AddNumbers("-5", "10"));
        Assert.AreEqual(0, Calculator.AddNumbers("0", "0"));
        // Add more test cases here
    }
}"#
    .trim()
    .to_string();

    (program_file_content, test_file_content)
}

pub fn generate_csharp_dsa_files() -> (String, String) {
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

pub fn generate_project_file() -> String {
    r#"<?xml version="1.0" encoding="utf-8"?>
<Project Sdk="Microsoft.NET.Sdk">

    <PropertyGroup>
        <OutputType>Exe</OutputType>
        <TargetFramework>net6.0</TargetFramework>
        <ImplicitUsings>enable</ImplicitUsings>
        <Nullable>enable</Nullable>
  </PropertyGroup>

  <ItemGroup>
        <PackageReference Include="Microsoft.NET.Test.Sdk" Version="16.11.0" />
        <PackageReference Include="NUnit" Version="3.13.0" />
        <PackageReference Include="NUnit3TestAdapter" Version="4.0.0" />
  </ItemGroup>

</Project>"#
        .trim()
        .to_string()
}
