# A WebAssembly Component Demo

The goal is to show how to create components, and use them to compose applications.

## A few words about components, their imports and exports, and the guest and host terminology

Something which has been very confusing to me to get right, is the terminoogy being used to describe imports and exports, and a component being a guest and/or a host. My clarification on this I've summarized below.

Components express the functionality they offer through exports. Components express the functionality they require through imports. This is tied to the WIT files, which components use.

It's not until you start composing applications, i.e., linking multiple compoments together, that a component can be describd as either a guest or host. This terminology is used to explain the relationship between two components in the dependency graph, not a component in isolation. E.g., Component A (which exports something), is being imported by component B. In that relationship, A is the guest, while B is the host.

Only exception being the Wasm runtime, which always is a host for the first component(s) in the dendency graph.

## The examples

In order to use a component, we need a Wasm rutime. As of the writing of this, the only way to use a components functionality, is to embed Wasmtime into a native program, import the component, and call it's exported functions. It's also not possible to link two or more components at runtime, they have to be "compiled" into a single wasm file. This is all pending updates to tooling and runtime, to make this more dynamic.

As the first example, I've wanted to build a component (add), which provide a function to add two numbers, then use this from a program (wasmtime-add) taking two arguments and returning the result.

As another example, to compose mulitple component together, I added an intermediate component (calculator), which is capabale of adding two numbers, by using the first component (add). Composing these two to a single component, which is hosted by a program (wasmtime-calculator).

## Building a component to add two numbers

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
