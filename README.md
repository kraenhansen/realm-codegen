# My Codegen Experiment

This is an experiment generating TypeScript types and and C++ binding code to interface with a C++ library.

## Prerequisites

- [Node.js & NPM](https://nodejs.org/en/download/)
- [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Note: I've only tested this on a Mac.

## Install the example

```
npm install --prefix tests/adder
```

## Compile the rust generator and generate some code

```
npm run generate --prefix tests/adder
```

## Compile the NAPI addon which consumes the generated code

```
npm run compile --prefix tests/adder
```

## Run the TypeScript test

```
npm test --prefix tests/adder
```
