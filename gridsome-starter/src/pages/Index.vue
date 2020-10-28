<template>
<div>
  <h1>
      Loading: {{ loading }}
  </h1>
  <button @click="greet">Greet me</button>
  <hr />


  <button @click="createUniverse">Create universe</button>
  </div>
</template>

<script>
const wasmImport = import('../index.js');

export default {
    methods: {
        greet() {
            this.wasm.greet();
        },
        createUniverse() {
            const uni = this.wasm.Universe.new();
            console.log({uni});
            const uniptr = uni.cells();

            const cells = new Uint8Array(this.wasm.memory.buffer, uniptr, 100);
            console.log({cells});
        }
    },
    created() {
        this.loading = true;
        wasmImport.then((r) => {
            this.loading = false;
            this.wasm = { ...r.default.wasm, ...r.default.wasm_bg };
            console.log(this.wasm);
        });
    },
    data() {
        return { 
            loading: false,
            wasm: null,
        }
    }
}
</script>

<style>

</style>