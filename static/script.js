// window.Module = {};
//
// function fetchAndInstantiate(url, importObject) {
//     return fetch(url)
//         .then(response => response.arrayBuffer())
//         .then(bytes => WebAssembly.instantiate(bytes, {env: importObject}))
//         .then(results => results.instance);
// }
// function newString(module, str) {
//     const utf8Encoder = new TextEncoder("UTF-8");
//     let string_buffer = utf8Encoder.encode(str)
//     let len = string_buffer.length
//     let ptr = module.alloc(len+1)
//
//     let memory = new Uint8Array(module.memory.buffer);
//     for (i = 0; i < len; i++) {
//         memory[ptr+i] = string_buffer[i]
//     }
//
//     memory[ptr+len] = 0;
//
//     return ptr;
// }
// function copyCStr(module, ptr) {
//     let orig_ptr = ptr;
//     const collectCString = function* () {
//         let memory = new Uint8Array(module.memory.buffer);
//         while (memory[ptr] !== 0) {
//             if (memory[ptr] === undefined) { throw new Error("Tried to read undef mem") }
//             yield memory[ptr]
//             ptr += 1
//         }
//     }
//
//     const buffer_as_u8 = new Uint8Array(collectCString())
//     const utf8Decoder = new TextDecoder("UTF-8");
//     const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
//     module.dealloc_str(orig_ptr);
//     return buffer_as_utf8
// }
//
// const memory = new WebAssembly.Memory({initial: 20});
// var imports = {
//     memory: memory,
//     javascript_fn: num => { alert(num); },
//     javascript_string_fn: (ptr, len) => {
//         var buf = new Uint8Array(memory.buffer, ptr, len)
//         var msg = new TextDecoder('utf8').decode(buf);
//         alert(msg);
//     }
// };
//
// fetchAndInstantiate('/rust_webassembly.wasm', imports)
//     .then(mod => {
//         console.log(mod.exports);
//
//         Module.memory = memory;
//         Module.alloc = mod.exports.alloc;
//         Module.dealloc = mod.exports.dealloc;
//         Module.dealloc_str = mod.exports.dealloc_str;
//         Module.main = mod.exports.main;
//         Module.test_string_return = (str) => {
//             let buf = newString(Module, str);
//             let outptr = mod.exports.test_string_return(buf);
//             let result = copyCStr(Module, outptr);
//             return result;
//         };
//         Module.test_string = mod.exports.test_string;
//         Module.test = mod.exports.test;
//
//         Module.main();
//
//         alert(Module.test_string_return("JAVASCRIPT"));
//        // Module.test_string(2);
//        // Module.test(0);
//     });