<template>
  <h1>
      Loading: {{ loading }}
  </h1>
  <button @click="greet">Greet me</button>
  <hr />

  <button @click="createUniverse">Create universe</button>
</template>

<script>
import wasmImport from './index.js';

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

        console.log({wasmImport});
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