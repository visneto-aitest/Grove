# Grove User Guide

## Introduction

Grove is a powerful coding agent harness designed to streamline development workflows. It integrates with various coding agents, allowing you to automate tasks, manage codebases, and interact with development tools efficiently. This guide will walk you through setting up Grove and using its features, including an example of how to leverage a "linear tool" within your workflows.

## Installation

To get started with Grove, follow these general steps:

1.  **Prerequisites**: Ensure you have Rust and Cargo installed on your system.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2.  **Clone the Repository**: Clone the Grove repository from its source.
    ```bash
    git clone <grove-repository-url>
    cd Grove
    ```
3.  **Build Grove**: Compile Grove using Cargo.
    ```bash
    cargo build --release
    ```
4.  **Run Grove**: Execute the compiled Grove application.
    ```bash
    ./target/release/grove
    ```

*(Note: Specific installation steps might vary based on your environment and the official Grove documentation. Please refer to the `README.md` or official project documentation for the most accurate instructions.)*

## Basic Usage

Grove interacts with coding agents and tools through a command-line interface or a TUI (Text User Interface). Here are some common actions:

*   **Listing Agents**: View available or active coding agents.
    ```
    # Example command (hypothetical)
    grove agents list
    ```
*   **Creating a New Agent**: Initialize a new coding agent for a specific task.
    ```
    # Example command (hypothetical)
    grove agent create --name "my-project-agent" --template "rust-dev"
    ```
*   **Sending Commands to an Agent**: Direct an agent to perform an action.
    ```
    # Example command (hypothetical)
    grove agent send --agent "my-project-agent" --command "run tests"
    ```

## Advanced Usage

Grove supports more complex workflows, including:

*   **Integrating with Pi-Coding Agent**: As described in `AGENTS.md`, Grove can integrate with pi-coding agents, enabling RPC-based communication for advanced agent control.
*   **Custom Tooling**: Define and integrate your own tools.
*   **Workflow Automation**: Script sequences of agent interactions for complex tasks.

## Understanding the "Linear Tool"

The concept of a "linear tool" within Grove (or any similar system) refers to a tool designed to process data or execute tasks in a sequential, step-by-step manner. It typically takes an input, performs a series of operations, and produces an output, often passing intermediate results from one step to the next. This linearity makes it predictable and easy to reason about, especially for pipelines where the order of operations is crucial.

### Characteristics of a Linear Tool:

*   **Sequential Execution**: Operations are performed in a defined order.
*   **Input-Output Chaining**: The output of one step often becomes the input for the next.
*   **Clear Stages**: Tasks are broken down into distinct, manageable stages.

### Example: A Hypothetical "Code Refactor" Linear Tool

Let's imagine a "Code Refactor" linear tool within Grove that automates a series of code quality improvements. This tool might take a file path as input and perform the following steps:

1.  **Format Code**: Apply standard code formatting rules.
2.  **Lint Check**: Run a linter to identify potential issues.
3.  **Apply Fixes**: Automatically apply safe fixes suggested by the linter.
4.  **Suggest Improvements**: (Optional) Suggest more complex improvements that require manual review.

#### How to use the "Code Refactor" Linear Tool:

Let's assume the linear tool is invoked via an agent command that specifies the tool and its arguments.

**Scenario 1: Refactoring a single file**

```
# Agent command to invoke the "code-refactor" linear tool on a specific file
grove agent send --agent "dev-agent" --tool "code-refactor" --args "src/main.rs"
```

In this example, the `dev-agent` is instructed to use the `code-refactor` tool. The tool will then process `src/main.rs` through its predefined linear steps (formatting, linting, applying fixes).

**Scenario 2: Refactoring multiple files in a directory (recursive)**

If the "code-refactor" tool supports directory input, it might apply its linear process to each file found.

```
# Agent command to invoke the "code-refactor" linear tool on an entire directory
grove agent send --agent "dev-agent" --tool "code-refactor" --args "src/" --recursive
```

Here, the `code-refactor` tool would iterate through all files in the `src/` directory and apply its linear sequence of operations to each.

**Scenario 3: Customizing the Linear Tool's Behavior**

Some linear tools might allow for configuration to skip certain steps or prioritize others.
/
```
# Agent command to invoke the "code-refactor" linear tool, skipping linting
grove agent send --agent "dev-agent" --tool "code-refactor" --args "src/utils.rs" --skip-step "lint"
```

In this case, the `dev-agent` uses the `code-refactor` tool on `src/utils.rs` but explicitly tells the tool to skip the "Lint Check" step, demonstrating how a linear tool, while sequential, can still offer some flexibility.

These examples illustrate how a "linear tool" can simplify complex, multi-step operations into a single, understandable command, making your development workflow more efficient and automated within the Grove ecosystem.
