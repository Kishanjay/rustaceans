# About
Starter template for Gridsome with WASM.

**Unfortunately gridsome doesn't support wasm at the time of writing**
## ERROR

```
UnhandledPromiseRejectionWarning: TypeError [ERR_INVALID_ARG_TYPE]: The "path" argument must be of type string. Received undefined
    at validateString (internal/validators.js:121:11)
    at join (path.js:1039:7)
    at assets/js/app.18f26473.js:92:25
    at new Promise (<anonymous>)
    at assets/js/app.18f26473.js:87:24
    at Array.forEach (<anonymous>)
    at Function.requireEnsure [as e] (assets/js/app.18f26473.js:79:24)
    at Module.iyQ6 (assets/js/page--src--pages--index-vue.db58f54a.js:140:38)
    at __webpack_require__ (assets/js/app.18f26473.js:47:30)
```

## WASM
Web assembly part has been setup according to these steps: [https://rustwasm.github.io/book/game-of-life/hello-world.html](https://rustwasm.github.io/book/game-of-life/hello-world.html)

```sh
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```