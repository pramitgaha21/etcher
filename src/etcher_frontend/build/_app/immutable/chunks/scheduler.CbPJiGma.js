function E(){}function C(t,e){for(const n in e)t[n]=e[n];return t}function D(t){return t()}function Z(){return Object.create(null)}function T(t){t.forEach(D)}function B(t){return typeof t=="function"}function $(t,e){return t!=t?e==e:t!==e||t&&typeof t=="object"||typeof t=="function"}function tt(t){return Object.keys(t).length===0}function L(t,...e){if(t==null){for(const i of e)i(void 0);return E}const n=t.subscribe(...e);return n.unsubscribe?()=>n.unsubscribe():n}function et(t,e,n){t.$$.on_destroy.push(L(e,n))}function nt(t,e,n,i){if(t){const c=N(t,e,n,i);return t[0](c)}}function N(t,e,n,i){return t[1]&&i?C(n.ctx.slice(),t[1](i(e))):n.ctx}function it(t,e,n,i){if(t[2]&&i){const c=t[2](i(n));if(e.dirty===void 0)return c;if(typeof c=="object"){const a=[],r=Math.max(e.dirty.length,c.length);for(let s=0;s<r;s+=1)a[s]=e.dirty[s]|c[s];return a}return e.dirty|c}return e.dirty}function ct(t,e,n,i,c,a){if(c){const r=N(e,n,i,a);t.p(r,c)}}function rt(t){if(t.ctx.length>32){const e=[],n=t.ctx.length/32;for(let i=0;i<n;i++)e[i]=-1;return e}return-1}function lt(t){const e={};for(const n in t)n[0]!=="$"&&(e[n]=t[n]);return e}function st(t,e){const n={};e=new Set(e);for(const i in t)!e.has(i)&&i[0]!=="$"&&(n[i]=t[i]);return n}function at(t){return t&&B(t.destroy)?t.destroy:E}let m=!1;function ot(){m=!0}function ut(){m=!1}function M(t,e,n,i){for(;t<e;){const c=t+(e-t>>1);n(c)<=i?t=c+1:e=c}return t}function q(t){if(t.hydrate_init)return;t.hydrate_init=!0;let e=t.childNodes;if(t.nodeName==="HEAD"){const l=[];for(let o=0;o<e.length;o++){const u=e[o];u.claim_order!==void 0&&l.push(u)}e=l}const n=new Int32Array(e.length+1),i=new Int32Array(e.length);n[0]=-1;let c=0;for(let l=0;l<e.length;l++){const o=e[l].claim_order,u=(c>0&&e[n[c]].claim_order<=o?c+1:M(1,c,P=>e[n[P]].claim_order,o))-1;i[l]=n[u]+1;const v=u+1;n[v]=l,c=Math.max(v,c)}const a=[],r=[];let s=e.length-1;for(let l=n[c]+1;l!=0;l=i[l-1]){for(a.push(e[l-1]);s>=l;s--)r.push(e[s]);s--}for(;s>=0;s--)r.push(e[s]);a.reverse(),r.sort((l,o)=>l.claim_order-o.claim_order);for(let l=0,o=0;l<r.length;l++){for(;o<a.length&&r[l].claim_order>=a[o].claim_order;)o++;const u=o<a.length?a[o]:null;t.insertBefore(r[l],u)}}function H(t,e){if(m){for(q(t),(t.actual_end_child===void 0||t.actual_end_child!==null&&t.actual_end_child.parentNode!==t)&&(t.actual_end_child=t.firstChild);t.actual_end_child!==null&&t.actual_end_child.claim_order===void 0;)t.actual_end_child=t.actual_end_child.nextSibling;e!==t.actual_end_child?(e.claim_order!==void 0||e.parentNode!==t)&&t.insertBefore(e,t.actual_end_child):t.actual_end_child=e.nextSibling}else(e.parentNode!==t||e.nextSibling!==null)&&t.appendChild(e)}function ft(t,e,n){m&&!n?H(t,e):(e.parentNode!==t||e.nextSibling!=n)&&t.insertBefore(e,n||null)}function _t(t){t.parentNode&&t.parentNode.removeChild(t)}function I(t){return document.createElement(t)}function z(t){return document.createElementNS("http://www.w3.org/2000/svg",t)}function x(t){return document.createTextNode(t)}function dt(){return x(" ")}function ht(){return x("")}function mt(t,e,n,i){return t.addEventListener(e,n,i),()=>t.removeEventListener(e,n,i)}function pt(t){return function(e){return e.preventDefault(),t.call(this,e)}}function bt(t){return function(e){return e.stopPropagation(),t.call(this,e)}}function j(t,e,n){n==null?t.removeAttribute(e):t.getAttribute(e)!==n&&t.setAttribute(e,n)}const F=["width","height"];function U(t,e){const n=Object.getOwnPropertyDescriptors(t.__proto__);for(const i in e)e[i]==null?t.removeAttribute(i):i==="style"?t.style.cssText=e[i]:i==="__value"?t.value=t[i]=e[i]:n[i]&&n[i].set&&F.indexOf(i)===-1?t[i]=e[i]:j(t,i,e[i])}function W(t,e){Object.keys(e).forEach(n=>{G(t,n,e[n])})}function G(t,e,n){const i=e.toLowerCase();i in t?t[i]=typeof t[i]=="boolean"&&n===""?!0:n:e in t?t[e]=typeof t[e]=="boolean"&&n===""?!0:n:j(t,e,n)}function yt(t){return/-/.test(t)?W:U}function gt(t){return t.dataset.svelteH}function xt(t){return Array.from(t.childNodes)}function J(t){t.claim_info===void 0&&(t.claim_info={last_index:0,total_claimed:0})}function A(t,e,n,i,c=!1){J(t);const a=(()=>{for(let r=t.claim_info.last_index;r<t.length;r++){const s=t[r];if(e(s)){const l=n(s);return l===void 0?t.splice(r,1):t[r]=l,c||(t.claim_info.last_index=r),s}}for(let r=t.claim_info.last_index-1;r>=0;r--){const s=t[r];if(e(s)){const l=n(s);return l===void 0?t.splice(r,1):t[r]=l,c?l===void 0&&t.claim_info.last_index--:t.claim_info.last_index=r,s}}return i()})();return a.claim_order=t.claim_info.total_claimed,t.claim_info.total_claimed+=1,a}function O(t,e,n,i){return A(t,c=>c.nodeName===e,c=>{const a=[];for(let r=0;r<c.attributes.length;r++){const s=c.attributes[r];n[s.name]||a.push(s.name)}a.forEach(r=>c.removeAttribute(r))},()=>i(e))}function wt(t,e,n){return O(t,e,n,I)}function vt(t,e,n){return O(t,e,n,z)}function K(t,e){return A(t,n=>n.nodeType===3,n=>{const i=""+e;if(n.data.startsWith(i)){if(n.data.length!==i.length)return n.splitText(i.length)}else n.data=i},()=>x(e),!0)}function kt(t){return K(t," ")}function Et(t,e){e=""+e,t.data!==e&&(t.data=e)}function Nt(t,e){t.value=e??""}function jt(t,e,n,i){n==null?t.style.removeProperty(e):t.style.setProperty(e,n,i?"important":"")}function Q(t,e,{bubbles:n=!1,cancelable:i=!1}={}){return new CustomEvent(t,{detail:e,bubbles:n,cancelable:i})}function At(t,e){return new t(e)}let h;function p(t){h=t}function w(){if(!h)throw new Error("Function called outside component initialization");return h}function Ot(t){w().$$.on_mount.push(t)}function St(t){w().$$.after_update.push(t)}function Pt(){const t=w();return(e,n,{cancelable:i=!1}={})=>{const c=t.$$.callbacks[e];if(c){const a=Q(e,n,{cancelable:i});return c.slice().forEach(r=>{r.call(t,a)}),!a.defaultPrevented}return!0}}function Ct(t,e){const n=t.$$.callbacks[e.type];n&&n.slice().forEach(i=>i.call(this,e))}const d=[],k=[];let _=[];const y=[],S=Promise.resolve();let g=!1;function R(){g||(g=!0,S.then(X))}function Dt(){return R(),S}function V(t){_.push(t)}function Tt(t){y.push(t)}const b=new Set;let f=0;function X(){if(f!==0)return;const t=h;do{try{for(;f<d.length;){const e=d[f];f++,p(e),Y(e.$$)}}catch(e){throw d.length=0,f=0,e}for(p(null),d.length=0,f=0;k.length;)k.pop()();for(let e=0;e<_.length;e+=1){const n=_[e];b.has(n)||(b.add(n),n())}_.length=0}while(d.length);for(;y.length;)y.pop()();g=!1,b.clear(),p(t)}function Y(t){if(t.fragment!==null){t.update(),T(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach(V)}}function Bt(t){const e=[],n=[];_.forEach(i=>t.indexOf(i)===-1?e.push(i):n.push(i)),n.forEach(i=>i()),_=e}export{Ct as $,X as A,tt as B,V as C,Bt as D,h as E,p as F,D as G,d as H,R as I,ot as J,ut as K,gt as L,mt as M,nt as N,ct as O,rt as P,it as Q,Pt as R,st as S,C as T,lt as U,U as V,at as W,z as X,vt as Y,bt as Z,pt as _,$ as a,Nt as a0,Tt as a1,yt as a2,dt as b,wt as c,xt as d,I as e,K as f,_t as g,kt as h,B as i,ft as j,H as k,Et as l,et as m,E as n,ht as o,St as p,Ot as q,T as r,L as s,x as t,j as u,jt as v,k as w,At as x,Dt as y,Z as z};
