import{createUrlDbWorker,execQuery}from"./snippets/sql-js-httpvfs-rs-7862d572fb3dc3a7/src/build/index.js";import*as __wbg_star0 from"./snippets/sql-js-httpvfs-rs-7862d572fb3dc3a7/src/build/index.js";let wasm;const heap=new Array(32).fill(void 0);function getObject(e){return heap[e]}heap.push(void 0,null,!0,!1);let heap_next=heap.length;function dropObject(e){e<36||(heap[e]=heap_next,heap_next=e)}function takeObject(e){const t=getObject(e);return dropObject(e),t}let cachedTextDecoder=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});cachedTextDecoder.decode();let cachegetUint8Memory0=null;function getUint8Memory0(){return null!==cachegetUint8Memory0&&cachegetUint8Memory0.buffer===wasm.memory.buffer||(cachegetUint8Memory0=new Uint8Array(wasm.memory.buffer)),cachegetUint8Memory0}function getStringFromWasm0(e,t){return cachedTextDecoder.decode(getUint8Memory0().subarray(e,e+t))}function addHeapObject(e){heap_next===heap.length&&heap.push(heap.length+1);const t=heap_next;return heap_next=heap[t],heap[t]=e,t}function debugString(e){const t=typeof e;if("number"==t||"boolean"==t||null==e)return`${e}`;if("string"==t)return`"${e}"`;if("symbol"==t){const t=e.description;return null==t?"Symbol":`Symbol(${t})`}if("function"==t){const t=e.name;return"string"==typeof t&&t.length>0?`Function(${t})`:"Function"}if(Array.isArray(e)){const t=e.length;let n="[";t>0&&(n+=debugString(e[0]));for(let r=1;r<t;r++)n+=", "+debugString(e[r]);return n+="]",n}const n=/\[object ([^\]]+)\]/.exec(toString.call(e));let r;if(!(n.length>1))return toString.call(e);if(r=n[1],"Object"==r)try{return"Object("+JSON.stringify(e)+")"}catch(e){return"Object"}return e instanceof Error?`${e.name}: ${e.message}\n${e.stack}`:r}let WASM_VECTOR_LEN=0,cachedTextEncoder=new TextEncoder("utf-8");const encodeString="function"==typeof cachedTextEncoder.encodeInto?function(e,t){return cachedTextEncoder.encodeInto(e,t)}:function(e,t){const n=cachedTextEncoder.encode(e);return t.set(n),{read:e.length,written:n.length}};function passStringToWasm0(e,t,n){if(void 0===n){const n=cachedTextEncoder.encode(e),r=t(n.length);return getUint8Memory0().subarray(r,r+n.length).set(n),WASM_VECTOR_LEN=n.length,r}let r=e.length,a=t(r);const c=getUint8Memory0();let o=0;for(;o<r;o++){const t=e.charCodeAt(o);if(t>127)break;c[a+o]=t}if(o!==r){0!==o&&(e=e.slice(o)),a=n(a,r,r=o+3*e.length);const t=getUint8Memory0().subarray(a+o,a+r);o+=encodeString(e,t).written}return WASM_VECTOR_LEN=o,a}let cachegetInt32Memory0=null;function getInt32Memory0(){return null!==cachegetInt32Memory0&&cachegetInt32Memory0.buffer===wasm.memory.buffer||(cachegetInt32Memory0=new Int32Array(wasm.memory.buffer)),cachegetInt32Memory0}function isLikeNone(e){return null==e}let cachegetFloat64Memory0=null;function getFloat64Memory0(){return null!==cachegetFloat64Memory0&&cachegetFloat64Memory0.buffer===wasm.memory.buffer||(cachegetFloat64Memory0=new Float64Array(wasm.memory.buffer)),cachegetFloat64Memory0}function makeMutClosure(e,t,n,r){const a={a:e,b:t,cnt:1,dtor:n},c=(...e)=>{a.cnt++;const t=a.a;a.a=0;try{return r(t,a.b,...e)}finally{0==--a.cnt?wasm.__wbindgen_export_2.get(a.dtor)(t,a.b):a.a=t}};return c.original=a,c}function __wbg_adapter_22(e,t,n){wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hfb73aae5a506fe24(e,t,addHeapObject(n))}function makeClosure(e,t,n,r){const a={a:e,b:t,cnt:1,dtor:n},c=(...e)=>{a.cnt++;try{return r(a.a,a.b,...e)}finally{0==--a.cnt&&(wasm.__wbindgen_export_2.get(a.dtor)(a.a,a.b),a.a=0)}};return c.original=a,c}function __wbg_adapter_25(e,t,n){wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcad17db096dbb908(e,t,addHeapObject(n))}let cachegetUint32Memory0=null;function getUint32Memory0(){return null!==cachegetUint32Memory0&&cachegetUint32Memory0.buffer===wasm.memory.buffer||(cachegetUint32Memory0=new Uint32Array(wasm.memory.buffer)),cachegetUint32Memory0}function getArrayJsValueFromWasm0(e,t){const n=getUint32Memory0().subarray(e/4,e/4+t),r=[];for(let e=0;e<n.length;e++)r.push(takeObject(n[e]));return r}function handleError(e,t){try{return e.apply(this,t)}catch(e){wasm.__wbindgen_exn_store(addHeapObject(e))}}async function load(e,t){if("function"==typeof Response&&e instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(e,t)}catch(t){if("application/wasm"==e.headers.get("Content-Type"))throw t;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",t)}const n=await e.arrayBuffer();return await WebAssembly.instantiate(n,t)}{const n=await WebAssembly.instantiate(e,t);return n instanceof WebAssembly.Instance?{instance:n,module:e}:n}}async function init(e){void 0===e&&(e=new URL("index-e73a4113c43b550_bg.wasm",import.meta.url));const t={wbg:{}};t.wbg.__wbindgen_object_drop_ref=function(e){takeObject(e)},t.wbg.__wbg_new_693216e109162396=function(){return addHeapObject(new Error)},t.wbg.__wbg_stack_0ddaca5d1abfb52f=function(e,t){var n=passStringToWasm0(getObject(t).stack,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),r=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=r,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_error_09919627ac0992f5=function(e,t){try{console.error(getStringFromWasm0(e,t))}finally{wasm.__wbindgen_free(e,t)}},t.wbg.__wbindgen_string_new=function(e,t){return addHeapObject(getStringFromWasm0(e,t))},t.wbg.__wbindgen_object_clone_ref=function(e){return addHeapObject(getObject(e))},t.wbg.__wbg_instanceof_Window_434ce1849eb4e0fc=function(e){return getObject(e)instanceof Window},t.wbg.__wbg_document_5edd43643d1060d9=function(e){var t=getObject(e).document;return isLikeNone(t)?0:addHeapObject(t)},t.wbg.__wbg_self_e23d74ae45fb17d1=function(){return handleError((function(){return addHeapObject(self.self)}),arguments)},t.wbg.__wbg_window_b4be7f48b24ac56e=function(){return handleError((function(){return addHeapObject(window.window)}),arguments)},t.wbg.__wbg_globalThis_d61b1f48a57191ae=function(){return handleError((function(){return addHeapObject(globalThis.globalThis)}),arguments)},t.wbg.__wbg_global_e7669da72fd7f239=function(){return handleError((function(){return addHeapObject(global.global)}),arguments)},t.wbg.__wbindgen_is_undefined=function(e){return void 0===getObject(e)},t.wbg.__wbg_newnoargs_f579424187aa1717=function(e,t){return addHeapObject(new Function(getStringFromWasm0(e,t)))},t.wbg.__wbg_call_89558c3e96703ca1=function(){return handleError((function(e,t){return addHeapObject(getObject(e).call(getObject(t)))}),arguments)},t.wbg.__wbg_set_c42875065132a932=function(){return handleError((function(e,t,n){return Reflect.set(getObject(e),getObject(t),getObject(n))}),arguments)},t.wbg.__wbindgen_json_parse=function(e,t){return addHeapObject(JSON.parse(getStringFromWasm0(e,t)))},t.wbg.__wbg_key_7f10b1291a923361=function(e,t){var n=passStringToWasm0(getObject(t).key,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),r=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=r,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_value_fc1c354d1a0e9714=function(e,t){var n=passStringToWasm0(getObject(t).value,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),r=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=r,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_createUrlDbWorker_7697b4c49ca87343=function(e,t,n,r,a,c){var o=getArrayJsValueFromWasm0(e,t).slice();return wasm.__wbindgen_free(e,4*t),addHeapObject(createUrlDbWorker(o,getStringFromWasm0(n,r),getStringFromWasm0(a,c)))},t.wbg.__wbg_execQuery_8e6920f1aae0deef=function(){return handleError((function(e,t){try{return addHeapObject(execQuery(getStringFromWasm0(e,t)))}finally{wasm.__wbindgen_free(e,t)}}),arguments)},t.wbg.__wbg_body_7538539844356c1c=function(e){var t=getObject(e).body;return isLikeNone(t)?0:addHeapObject(t)},t.wbg.__wbg_lastChild_e2b014abab089e08=function(e){var t=getObject(e).lastChild;return isLikeNone(t)?0:addHeapObject(t)},t.wbg.__wbg_createTextNode_39a0de25d14bcde5=function(e,t,n){return addHeapObject(getObject(e).createTextNode(getStringFromWasm0(t,n)))},t.wbg.__wbg_focus_4434360545ac99cf=function(){return handleError((function(e){getObject(e).focus()}),arguments)},t.wbg.__wbindgen_debug_string=function(e,t){var n=passStringToWasm0(debugString(getObject(t)),wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),r=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=r,getInt32Memory0()[e/4+0]=n},t.wbg.__wbindgen_throw=function(e,t){throw new Error(getStringFromWasm0(e,t))},t.wbg.__wbg_then_58a04e42527f52c6=function(e,t,n){return addHeapObject(getObject(e).then(getObject(t),getObject(n)))},t.wbg.__wbg_then_a6860c82b90816ca=function(e,t){return addHeapObject(getObject(e).then(getObject(t)))},t.wbg.__wbg_resolve_4f8f547f26b30b27=function(e){return addHeapObject(Promise.resolve(getObject(e)))},t.wbg.__wbindgen_cb_drop=function(e){const t=takeObject(e).original;if(1==t.cnt--)return t.a=0,!0;return!1},t.wbg.__wbg_setAttribute_1776fcc9b98d464e=function(){return handleError((function(e,t,n,r,a){getObject(e).setAttribute(getStringFromWasm0(t,n),getStringFromWasm0(r,a))}),arguments)},t.wbg.__wbg_debug_6e114a5b27d7915d=function(e){console.debug(getObject(e))},t.wbg.__wbg_error_ca520cb687b085a1=function(e){console.error(getObject(e))},t.wbg.__wbg_info_32ab782ec7072fac=function(e){console.info(getObject(e))},t.wbg.__wbg_log_fbd13631356d44e4=function(e){console.log(getObject(e))},t.wbg.__wbg_warn_97f10a6b0dbb8c5c=function(e){console.warn(getObject(e))},t.wbg.__wbg_removeChild_f4a83c9698136bbb=function(){return handleError((function(e,t){return addHeapObject(getObject(e).removeChild(getObject(t)))}),arguments)},t.wbg.__wbindgen_number_new=function(e){return addHeapObject(e)},t.wbg.__wbg_setvalue_ce4a23f487065c07=function(e,t,n){getObject(e).value=getStringFromWasm0(t,n)},t.wbg.__wbg_setvalue_6a34bab301f38bf2=function(e,t,n){getObject(e).value=getStringFromWasm0(t,n)},t.wbg.__wbg_value_d3a30bc2c7caf357=function(e,t){var n=passStringToWasm0(getObject(t).value,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),r=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=r,getInt32Memory0()[e/4+0]=n},t.wbg.__wbg_namespaceURI_e9a971e6c1ce68db=function(e,t){var n=getObject(t).namespaceURI,r=isLikeNone(n)?0:passStringToWasm0(n,wasm.__wbindgen_malloc,wasm.__wbindgen_realloc),a=WASM_VECTOR_LEN;getInt32Memory0()[e/4+1]=a,getInt32Memory0()[e/4+0]=r},t.wbg.__wbg_createElement_d017b8d2af99bab9=function(){return handleError((function(e,t,n){return addHeapObject(getObject(e).createElement(getStringFromWasm0(t,n)))}),arguments)},t.wbg.__wbg_createElementNS_fd4a7e49f74039e1=function(){return handleError((function(e,t,n,r,a){return addHeapObject(getObject(e).createElementNS(0===t?void 0:getStringFromWasm0(t,n),getStringFromWasm0(r,a)))}),arguments)},t.wbg.__wbg_appendChild_3fe5090c665d3bb4=function(){return handleError((function(e,t){return addHeapObject(getObject(e).appendChild(getObject(t)))}),arguments)},t.wbg.__wbg_insertBefore_4f09909023feac91=function(){return handleError((function(e,t,n){return addHeapObject(getObject(e).insertBefore(getObject(t),getObject(n)))}),arguments)},t.wbg.__wbg_warn_2aa0e7178e1d35f6=function(e,t){var n=getArrayJsValueFromWasm0(e,t).slice();wasm.__wbindgen_free(e,4*t),console.warn(...n)},t.wbg.__wbg_setchecked_f6ead3490df88a7f=function(e,t){getObject(e).checked=0!==t},t.wbg.__wbg_setnodeValue_f175b74a390f8fda=function(e,t,n){getObject(e).nodeValue=0===t?void 0:getStringFromWasm0(t,n)},t.wbg.__wbg_is_3d73f4d91adacc37=function(e,t){return Object.is(getObject(e),getObject(t))},t.wbg.__wbg_removeAttribute_1adaecf6b4d35a09=function(){return handleError((function(e,t,n){getObject(e).removeAttribute(getStringFromWasm0(t,n))}),arguments)},t.wbg.__wbg_new_d3138911a89329b0=function(){return addHeapObject(new Object)},t.wbg.__wbg_addEventListener_55682f77717d7665=function(){return handleError((function(e,t,n,r,a){getObject(e).addEventListener(getStringFromWasm0(t,n),getObject(r),getObject(a))}),arguments)},t.wbg.__wbg_target_e560052e31e4567c=function(e){var t=getObject(e).target;return isLikeNone(t)?0:addHeapObject(t)},t.wbg.__wbg_instanceof_Element_c9423704dd5d9b1d=function(e){return getObject(e)instanceof Element},t.wbg.__wbg_cancelBubble_17d7988ab2fbe4c9=function(e){return getObject(e).cancelBubble},t.wbg.__wbg_parentElement_96e1e07348340043=function(e){var t=getObject(e).parentElement;return isLikeNone(t)?0:addHeapObject(t)},t.wbg.__wbg_get_8bbb82393651dd9c=function(){return handleError((function(e,t){return addHeapObject(Reflect.get(getObject(e),getObject(t)))}),arguments)},t.wbg.__wbindgen_number_get=function(e,t){const n=getObject(t);var r="number"==typeof n?n:void 0;getFloat64Memory0()[e/8+1]=isLikeNone(r)?0:r,getInt32Memory0()[e/4+0]=!isLikeNone(r)},t.wbg.__wbg_valueOf_39e0d6bc7e4232b9=function(e){return getObject(e).valueOf()},t.wbg.__wbindgen_closure_wrapper1282=function(e,t,n){return addHeapObject(makeMutClosure(e,t,43,__wbg_adapter_22))},t.wbg.__wbindgen_closure_wrapper3153=function(e,t,n){return addHeapObject(makeClosure(e,t,48,__wbg_adapter_25))},t["./snippets/sql-js-httpvfs-rs-7862d572fb3dc3a7/src/build/index.js"]=__wbg_star0,("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e));const{instance:n,module:r}=await load(await e,t);return wasm=n.exports,init.__wbindgen_wasm_module=r,wasm.__wbindgen_start(),wasm}export default init;
