# CWJH WASM Tools

## Installation

Install with npm/yarn from registry
`yarn add @klos71/cwjh_wasm`

## Usage

init wasm like a js module in your project then init wasm and call upon commands that exists

### Example usage

-   [cwjh_tools](https://github.com/klos71/cwjh-tools/)

## Local development

install [wasm-pack](https://rustwasm.github.io/wasm-pack/) on your local machine

to install the dependencies
`cargo build`

build the pkg folder containing the js module with `wasm-pack build --scope klos71 --release --target web` when the pkg folder has been generated cd into it and `yarn link` to create a local yarn package and then use `yarn link "@klos71/cwjh_wasm"` in the web project.
