# prek-test-repos/rust-hooks

Minimal Rust hooks for testing [prek](https://github.com/j178/prek) Rust language support.

## Hooks

### hello-world

A simple hook that echoes its arguments.

### hello-world-lib-deps

A hook that requires `itoa` to be added via `additional_dependencies`.
Used to test that library dependencies work correctly without modifying the shared repository.

## Usage

```yaml
repos:
  - repo: https://github.com/prek-test-repos/rust-hooks
    rev: v1.0.0
    hooks:
      - id: hello-world
        args: ["Hello World"]
      - id: hello-world-lib-deps
        additional_dependencies: ["itoa:1"]
```
