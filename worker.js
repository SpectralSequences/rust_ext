!function(e){self.webpackChunk=function(n,r){for(var o in r)e[o]=r[o];for(;n.length;)t[n.pop()]=1};var n={},t={0:1},r={};var o={2:function(){return{"./index.js":{__wbindgen_object_drop_ref:function(e){return n[1].exports.__wbindgen_object_drop_ref(e)},__wbindgen_number_new:function(e){return n[1].exports.__wbindgen_number_new(e)},__wbindgen_string_new:function(e,t){return n[1].exports.__wbindgen_string_new(e,t)},__wbg_new_f1f0f3113e466334:function(){return n[1].exports.__wbg_new_f1f0f3113e466334()},__wbg_push_829cf1fbae322d44:function(e,t){return n[1].exports.__wbg_push_829cf1fbae322d44(e,t)},__wbg_apply_a7d91e1867ff2ba0:function(e,t,r){return n[1].exports.__wbg_apply_a7d91e1867ff2ba0(e,t,r)},__wbg_call_213fbeddd38b2990:function(e,t,r,o,s){return n[1].exports.__wbg_call_213fbeddd38b2990(e,t,r,o,s)},__wbindgen_debug_string:function(e,t){return n[1].exports.__wbindgen_debug_string(e,t)},__wbindgen_throw:function(e,t){return n[1].exports.__wbindgen_throw(e,t)}}}}};function s(t){if(n[t])return n[t].exports;var r=n[t]={i:t,l:!1,exports:{}};return e[t].call(r.exports,r,r.exports,s),r.l=!0,r.exports}s.e=function(e){var n=[];return n.push(Promise.resolve().then(function(){t[e]||importScripts(s.p+""+e+".worker.js")})),({1:[2]}[e]||[]).forEach(function(e){var t=r[e];if(t)n.push(t);else{var l,f=o[e](),i=fetch(s.p+""+{2:"3885e8135b95c934e8cc"}[e]+".module.wasm");if(f instanceof Promise&&"function"==typeof WebAssembly.compileStreaming)l=Promise.all([WebAssembly.compileStreaming(i),f]).then(function(e){return WebAssembly.instantiate(e[0],e[1])});else if("function"==typeof WebAssembly.instantiateStreaming)l=WebAssembly.instantiateStreaming(i,f);else{l=i.then(function(e){return e.arrayBuffer()}).then(function(e){return WebAssembly.instantiate(e,f)})}n.push(r[e]=l.then(function(n){return s.w[e]=(n.instance||n).exports}))}}),Promise.all(n)},s.m=e,s.c=n,s.d=function(e,n,t){s.o(e,n)||Object.defineProperty(e,n,{enumerable:!0,get:t})},s.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},s.t=function(e,n){if(1&n&&(e=s(e)),8&n)return e;if(4&n&&"object"==typeof e&&e&&e.__esModule)return e;var t=Object.create(null);if(s.r(t),Object.defineProperty(t,"default",{enumerable:!0,value:e}),2&n&&"string"!=typeof e)for(var r in e)s.d(t,r,function(n){return e[n]}.bind(null,r));return t},s.n=function(e){var n=e&&e.__esModule?function(){return e.default}:function(){return e};return s.d(n,"a",n),n},s.o=function(e,n){return Object.prototype.hasOwnProperty.call(e,n)},s.p="",s.w={},s(s.s=0)}([function(e,n,t){let r=t.e(1).then(t.bind(null,1)).catch(console.error).then(e=>{self.wasm=e});function o(e,n,t){n>self.max_int_deg&&(self.max_int_deg=n,self.max_int_deg%10==0&&(console.log(`Time to compute stems ${self.max_int_deg-10} to ${self.max_int_deg} : ${function(){let e=performance.now(),n=(e-f)/1e3;return f=e,n}()}`),console.log(`Total time to compute first ${self.max_int_deg} stems : ${i()}`))),self.postMessage({cmd:"addClass",x:n-e,y:e})}function s(e,n,t,r,o,s,l){self.postMessage({cmd:"addStructline",mult:e,source:{x:t-n,y:n,idx:r},target:{x:s-o,y:o,idx:l}})}self.max_int_deg=0;let l=performance.now(),f=l;function i(){return(performance.now()-l)/1e3}self.onmessage=e=>{if(!self.wasm)return void r.then(()=>self.onmessage(e));let n=e.data;n.cmd in a?(a[n.cmd](n),self.postMessage({cmd:"complete",data:n})):console.error(`Unknown command '${n.cmd}'`)};let a={resolve:function(e){self.p=e.p,self.algebra=self.wasm.WasmAlgebra.new_adem_algebra(e.p,2!=e.p,e.maxDegree),self.algebra.compute_basis(e.maxDegree),self.fdmodule=self.wasm.WasmModule.new_adem_module(algebra,e.module),self.cc=self.wasm.WasmChainComplex.new_ccdz(fdmodule),self.res=self.wasm.WasmResolution.new(cc,e.maxDegree,o,s),self.res.resolve_through_degree(e.maxDegree),console.log(`Total time : ${i()}`)},getCocycle:function(e){if(!self.res)return void console.log("No resolution yet, can't get cocycle");let n=e.class.y,t=e.class.x+e.class.y,r=self.res.get_cocycle_string(n,t,e.class.idx);2==self.p&&(r=r.replace(/P/g,"Sq")),self.postMessage({cmd:"cocycleResult",class:e.class,cocycle:r})}}}]);