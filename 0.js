self.webpackChunk([0],[function(t,n,r){"use strict";r.r(n),r.d(n,"Resolution",function(){return y}),r.d(n,"Sseq",function(){return m}),r.d(n,"SteenrodCalculator",function(){return j}),r.d(n,"__wbindgen_object_drop_ref",function(){return O}),r.d(n,"__wbindgen_string_new",function(){return S}),r.d(n,"__wbindgen_object_clone_ref",function(){return $}),r.d(n,"__wbg_call_0ec43f2615658695",function(){return v}),r.d(n,"__wbindgen_debug_string",function(){return A}),r.d(n,"__wbindgen_throw",function(){return k}),r.d(n,"__wbindgen_rethrow",function(){return x});var e=r(4);const c=new Array(32);c.fill(void 0),c.push(void 0,null,!0,!1);let o=c.length;function u(t){o===c.length&&c.push(c.length+1);const n=o;return o=c[n],c[n]=t,n}let i=0,s=new TextEncoder("utf-8");const f="function"==typeof s.encodeInto?function(t,n){return s.encodeInto(t,n)}:function(t,n){const r=s.encode(t);return n.set(r),{read:t.length,written:r.length}};let l=null;function a(){return null!==l&&l.buffer===e.h.buffer||(l=new Uint8Array(e.h.buffer)),l}function _(t){let n=t.length,r=e.f(n);const c=a();let o=0;for(;o<n;o++){const n=t.charCodeAt(o);if(n>127)break;c[r+o]=n}if(o!==n){0!==o&&(t=t.slice(o)),r=e.g(r,n,n=o+3*t.length);const c=a().subarray(r+o,r+n);o+=f(t,c).written}return i=o,r}let p=null;function h(){return null!==p&&p.buffer===e.h.buffer||(p=new Int32Array(e.h.buffer)),p}let b=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});function d(t,n){return b.decode(a().subarray(t,t+n))}function w(t){return c[t]}function g(t){const n=w(t);return function(t){t<36||(c[t]=o,o=t)}(t),n}class y{static __wrap(t){const n=Object.create(y.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,e.a(t)}static new(t){const n=e.i(u(t));return y.__wrap(n)}run(t){e.j(this.ptr,_(t),i)}}class m{static __wrap(t){const n=Object.create(m.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,e.b(t)}static new(t){const n=e.k(u(t));return m.__wrap(n)}run(t){e.l(this.ptr,_(t),i)}}class j{static __wrap(t){const n=Object.create(j.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,e.c(t)}static new(t){const n=e.q(t);return j.__wrap(n)}compute_basis(t){e.m(this.ptr,t)}evaluate_adem(t){e.n(8,this.ptr,_(t),i);const n=h(),r=d(n[2],n[3]).slice();return e.e(n[2],1*n[3]),r}evaluate_milnor(t){e.o(8,this.ptr,_(t),i);const n=h(),r=d(n[2],n[3]).slice();return e.e(n[2],1*n[3]),r}free(){const t=this.ptr;this.ptr=0,e.p(t)}}const O=function(t){g(t)},S=function(t,n){return u(d(t,n))},$=function(t){return u(w(t))},v=function(t,n,r){try{return u(w(t).call(w(n),w(r)))}catch(t){!function(t){e.d(u(t))}(t)}},A=function(t,n){const r=_(function t(n){const r=typeof n;if("number"==r||"boolean"==r||null==n)return`${n}`;if("string"==r)return`"${n}"`;if("symbol"==r){const t=n.description;return null==t?"Symbol":`Symbol(${t})`}if("function"==r){const t=n.name;return"string"==typeof t&&t.length>0?`Function(${t})`:"Function"}if(Array.isArray(n)){const r=n.length;let e="[";r>0&&(e+=t(n[0]));for(let c=1;c<r;c++)e+=", "+t(n[c]);return e+="]"}const e=/\[object ([^\]]+)\]/.exec(toString.call(n));let c;if(!(e.length>1))return toString.call(n);if("Object"==(c=e[1]))try{return"Object("+JSON.stringify(n)+")"}catch(t){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:c}(w(n))),e=i;h()[t/4+0]=r,h()[t/4+1]=e},k=function(t,n){throw new Error(d(t,n))},x=function(t){throw g(t)}},,,,function(t,n,r){"use strict";var e=r.w[t.i];t.exports=e;r(0);e.r()}]);