# Kata Machine

A quick way to generate a sandbox, start coding and easily test your katas across multiple languages

## Commands

- init

    Ensures you have everything required to run the tests installed, and if not, it will install it.

    It will check/install:

    - dotnet (will install .NET6 if not)

    - npm (will install latest)

    - rust (will install the latest)

- generate

    Generates the kata for the day. You can specify the languages and katas listed below.
    Also generates a session.json file, which tracks the most recent day (used below).

    ### Languages

    - `rust` _(default)_

    - `csharp` or `cs`

    - `typescript` or `ts`

    ### Katas

    - `calculator` - based on Roy Osherov's [String Calculator](https://osherove.com/string-calculator) _(default)_

    - `dsa` - coming soon, will generate a project of data structures and algorithms that use it to be solved

    - `interview` - coming soon, will generate a interview-like question from a small pool of examples

- test

    Tests the most recent day using the session.json created during `generate`.
    
    Note: This is simply a shorthand for running `cargo test`, `dotnet test`, or `npm test` depending on the language.
    If you want to continue a previous day, simply run the command in the folder and it will work the same.

- clean

    Cleans up all Kata Machine data. This includes:

    - All language folders, including all their content

    - The session.json

    - Removing .vscode/settings.json