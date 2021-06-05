/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunkrose_web"] = self["webpackChunkrose_web"] || []).push([["index_js"],{

/***/ "../rose-wasm/pkg/rose_wasm_bg.js":
/*!****************************************!*\
  !*** ../rose-wasm/pkg/rose_wasm_bg.js ***!
  \****************************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"get_svg\": () => (/* binding */ get_svg)\n/* harmony export */ });\n/* harmony import */ var _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./rose_wasm_bg.wasm */ \"../rose-wasm/pkg/rose_wasm_bg.wasm\");\n/* module decorator */ module = __webpack_require__.hmd(module);\n\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {\n        cachegetInt32Memory0 = new Int32Array(_rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* @param {number} num_generations\n* @param {string} seed\n* @param {string} color_scheme\n* @param {number} stroke_width\n* @param {boolean} draw_triangles\n* @param {boolean} draw_arcs\n* @returns {string}\n*/\nfunction get_svg(num_generations, seed, color_scheme, stroke_width, draw_triangles, draw_arcs) {\n    try {\n        const retptr = _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_export_0.value - 16;\n        _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_export_0.value = retptr;\n        var ptr0 = passStringToWasm0(seed, _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n        var len0 = WASM_VECTOR_LEN;\n        var ptr1 = passStringToWasm0(color_scheme, _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n        var len1 = WASM_VECTOR_LEN;\n        _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.get_svg(retptr, num_generations, ptr0, len0, ptr1, len1, stroke_width, draw_triangles, draw_arcs);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_export_0.value += 16;\n        _rose_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1);\n    }\n}\n\n\n\n//# sourceURL=webpack://rose-web/../rose-wasm/pkg/rose_wasm_bg.js?");

/***/ }),

/***/ "../rose-wasm/pkg/rose_wasm_bg.wasm":
/*!******************************************!*\
  !*** ../rose-wasm/pkg/rose_wasm_bg.wasm ***!
  \******************************************/
/***/ ((module, exports, __webpack_require__) => {

"use strict";
eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.id];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name) exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"\"]()\n\n//# sourceURL=webpack://rose-web/../rose-wasm/pkg/rose_wasm_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var rose_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! rose-wasm */ \"../rose-wasm/pkg/rose_wasm_bg.js\");\n\n\nconst svg_container = document.getElementById(\"svg-container\");\n\nconst generate = () => {\n    let num_generations = +document.getElementById(\"input-num-generations\").value;\n    let seed = document.getElementById(\"input-seed\").value;\n    let color_scheme = document.getElementById(\"input-color-scheme\").value;\n    let stroke_width = +document.getElementById(\"input-stroke-width\").value;\n    let draw_triangles = !!document.getElementById(\"input-draw-triangles\").checked;\n    let draw_arcs = !!document.getElementById(\"input-draw-arcs\").checked;\n    \n    let svg = rose_wasm__WEBPACK_IMPORTED_MODULE_0__.get_svg(\n        num_generations,\n        seed,\n        color_scheme,\n        stroke_width,\n        draw_triangles,\n        draw_arcs,\n    );\n\n    let svg_container = document.getElementById(\"svg-container\");\n    svg_container.innerHTML = svg;\n    svg_container.children[0].style.transform = `scale(${scale})`;\n};\n\ndocument.getElementById(\"button-generate\").onclick = generate;\n\nlet scale = 1.0;\nconst zoom = (event) => {\n    event.preventDefault();\n    const zoom_sensitivity = 0.04;\n    scale += event.deltaY * -zoom_sensitivity * (scale - 0.3);\n    scale = Math.min(Math.max(1.0, scale), 16.0);\n    svg_container.children[0].style.transform = `scale(${scale})`;\n};\n\nsvg_container.onwheel = zoom;\ngenerate();\n\n\n//# sourceURL=webpack://rose-web/./index.js?");

/***/ })

}]);