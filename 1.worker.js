self.webpackChunk([1],[,function(t,n,r){"use strict";r.r(n),r.d(n,"WasmAlgebra",function(){return m}),r.d(n,"WasmCCDZ",function(){return j}),r.d(n,"WasmModule",function(){return A}),r.d(n,"WasmResolution",function(){return O}),r.d(n,"__wbindgen_object_drop_ref",function(){return $}),r.d(n,"__wbindgen_number_new",function(){return k}),r.d(n,"__wbindgen_string_new",function(){return v}),r.d(n,"__wbg_new_db45baf1426052b4",function(){return x}),r.d(n,"__wbg_push_d01e624d96b4cd82",function(){return C}),r.d(n,"__wbg_apply_df841622788415e8",function(){return S}),r.d(n,"__wbg_call_445af161ae71e490",function(){return E}),r.d(n,"__wbindgen_debug_string",function(){return W}),r.d(n,"__wbindgen_throw",function(){return I});var e=r(2);function c(t,n){if(!(t instanceof n))throw new Error(`expected instance of ${n.name}`);return t.ptr}let o,u=0,i=new TextEncoder("utf-8"),s=null;function f(){return null!==s&&s.buffer===e.i.buffer||(s=new Uint8Array(e.i.buffer)),s}o="function"==typeof i.encodeInto?function(t){let n=t.length,r=e.g(n),c=0;{const n=f();for(;c<t.length;c++){const e=t.charCodeAt(c);if(e>127)break;n[r+c]=e}}if(c!==t.length){t=t.slice(c),r=e.h(r,n,n=c+3*t.length);const o=f().subarray(r+c,r+n);c+=i.encodeInto(t,o).written}return u=c,r}:function(t){let n=t.length,r=e.g(n),c=0;{const n=f();for(;c<t.length;c++){const e=t.charCodeAt(c);if(e>127)break;n[r+c]=e}}if(c!==t.length){const o=i.encode(t.slice(c));r=e.h(r,n,n=c+o.length),f().set(o,r+c),c+=o.length}return u=c,r};const a=new Array(32);a.fill(void 0),a.push(void 0,null,!0,!1);let l=a.length;function _(t){l===a.length&&a.push(a.length+1);const n=l;return l=a[n],a[n]=t,n}let p=null;function h(){return null!==p&&p.buffer===e.i.buffer||(p=new Int32Array(e.i.buffer)),p}let b=new TextDecoder("utf-8");function d(t,n){return b.decode(f().subarray(t,t+n))}function g(t){return a[t]}function w(t){const n=g(t);return function(t){t<36||(a[t]=l,l=t)}(t),n}function y(t){e.e(_(t))}class m{static __wrap(t){const n=Object.create(m.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,e.a(t)}static new_adem_algebra(t,n){const r=e.l(t,n);return m.__wrap(r)}static new_milnor_algebra(t){const n=e.m(t);return m.__wrap(n)}compute_basis(t){e.j(this.ptr,t)}free(){const t=this.ptr;this.ptr=0,e.k(t)}}class j{static __wrap(t){const n=Object.create(j.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,e.b(t)}static new_ccdz(t){c(t,A);const n=e.o(t.ptr);return j.__wrap(n)}free(){const t=this.ptr;this.ptr=0,e.n(t)}}class A{static __wrap(t){const n=Object.create(A.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,e.c(t)}static new_adem_module(t,n){c(t,m);const r=e.q(t.ptr,o(n),u);return A.__wrap(r)}free(){const t=this.ptr;this.ptr=0,e.p(t)}}class O{static __wrap(t){const n=Object.create(O.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,e.d(t)}static new(t,n,r,i){c(t,j);const s=e.t(t.ptr,o(n),u,_(r),_(i));return O.__wrap(s)}resolve_through_degree(t){e.u(this.ptr,t)}get_cocycle_string(t,n,r){e.s(8,this.ptr,t,n,r);const c=h(),o=d(c[2],c[3]).slice();return e.f(c[2],1*c[3]),o}free(){const t=this.ptr;this.ptr=0,e.r(t)}}const $=function(t){w(t)},k=function(t){return _(t)},v=function(t,n){return _(d(t,n))},x=function(){return _(new Array)},C=function(t,n){return g(t).push(g(n))},S=function(t,n,r){try{return _(g(t).apply(g(n),g(r)))}catch(t){y(t)}},E=function(t,n,r,e){try{return _(g(t).call(g(n),g(r),g(e)))}catch(t){y(t)}},W=function(t,n){const r=function t(n){const r=typeof n;if("number"==r||"boolean"==r||null==n)return`${n}`;if("string"==r)return`"${n}"`;if("symbol"==r){const t=n.description;return null==t?"Symbol":`Symbol(${t})`}if("function"==r){const t=n.name;return"string"==typeof t&&t.length>0?`Function(${t})`:"Function"}if(Array.isArray(n)){const r=n.length;let e="[";r>0&&(e+=t(n[0]));for(let c=1;c<r;c++)e+=", "+t(n[c]);return e+="]"}const e=/\[object ([^\]]+)\]/.exec(toString.call(n));let c;if(!(e.length>1))return toString.call(n);if("Object"==(c=e[1]))try{return"Object("+JSON.stringify(n)+")"}catch(t){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:c}(g(n)),e=o(r),c=u;h()[t/4+0]=e,h()[t/4+1]=c},I=function(t,n){throw new Error(d(t,n))}},function(t,n,r){"use strict";var e=r.w[t.i];t.exports=e;r(1);e.v()}]);