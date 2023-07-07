# A WebAssemblt Component Demo

The goal is to show how to compose applications using various components.

Components express the functionality they offer through exports.
Components express the functionality they require through imports.

It's not until you start composing applications, using components, that they become either guests or hosts. This is mainly a terminology that can be used to explain the relationship between two components in the dependency graph, not a component in isolation. Only exception being the Wasm runtime, which always is a host for the first component(s) in the dendency graph.

## Build a component to add two numbers

1. Define the [WIT](./add/wit/add.wit) definition for the component.
2. Implement the function, using `wit-bindgen` to generate the bindings for the WIT World.
3. Build a module `cargo b -target=wasm32-unknown-unknown` - not using WASI, as this will import the WASI World (a bunch of interfaces).
4. Create a component form the module `wasm-tools component new target/wasm32-unknown-unknown/debug/add.wasm -o add.wasm`
5. Check the component `wasm-tools component wit add.wasm`

## Build a native Rust program to host the component using Wasmtime

We cannot dynamically run the component using Wasmtime CLI (not yet supported), so have to build a program to host it.

1. Use Wasmtime to instantiate the component, and import the WIT file for that component.
2. `cargo run`

## Build an intermediate component - a calculator, which imports the add operator, and exports a calc function

1. Define the [WIT](./calculator/wit/calculator.wit) definition for the component.
2. Implement the function, using `wit-bindgen` to generate the bindings for the WIT World.
3. Build a module `cargo b -target=wasm32-unknown-unknown` - not using WASI, as this will import the WASI World (a bunch of interfaces).
4. Create a component form the module `wasm-tools component new target/wasm32-unknown-unknown/debug/calculator.wasm -o calculator.wasm`
5. Check the component `wasm-tools component wit calculator.wasm`

## Build a native Rust program to host the component using Wasmtime

We cannot dynamically run the component using Wasmtime CLI (not yet supported), so have to build a program to host it.

1. First built a new component from the two existing components, using https://wasmbuilder.app/, remember to export the exports of the `calculator` component as exports for the new component.
2. Use Wasmtime to instantiate the component, and import the WIT file for the `calculator` component.
3. `cargo run`