(()=>{"use strict";var e,c,a,d,t,f={},r={};function b(e){var c=r[e];if(void 0!==c)return c.exports;var a=r[e]={id:e,loaded:!1,exports:{}};return f[e].call(a.exports,a,a.exports,b),a.loaded=!0,a.exports}b.m=f,b.c=r,e=[],b.O=(c,a,d,t)=>{if(!a){var f=1/0;for(i=0;i<e.length;i++){a=e[i][0],d=e[i][1],t=e[i][2];for(var r=!0,o=0;o<a.length;o++)(!1&t||f>=t)&&Object.keys(b.O).every((e=>b.O[e](a[o])))?a.splice(o--,1):(r=!1,t<f&&(f=t));if(r){e.splice(i--,1);var n=d();void 0!==n&&(c=n)}}return c}t=t||0;for(var i=e.length;i>0&&e[i-1][2]>t;i--)e[i]=e[i-1];e[i]=[a,d,t]},b.n=e=>{var c=e&&e.__esModule?()=>e.default:()=>e;return b.d(c,{a:c}),c},a=Object.getPrototypeOf?e=>Object.getPrototypeOf(e):e=>e.__proto__,b.t=function(e,d){if(1&d&&(e=this(e)),8&d)return e;if("object"==typeof e&&e){if(4&d&&e.__esModule)return e;if(16&d&&"function"==typeof e.then)return e}var t=Object.create(null);b.r(t);var f={};c=c||[null,a({}),a([]),a(a)];for(var r=2&d&&e;"object"==typeof r&&!~c.indexOf(r);r=a(r))Object.getOwnPropertyNames(r).forEach((c=>f[c]=()=>e[c]));return f.default=()=>e,b.d(t,f),t},b.d=(e,c)=>{for(var a in c)b.o(c,a)&&!b.o(e,a)&&Object.defineProperty(e,a,{enumerable:!0,get:c[a]})},b.f={},b.e=e=>Promise.all(Object.keys(b.f).reduce(((c,a)=>(b.f[a](e,c),c)),[])),b.u=e=>"assets/js/"+({53:"935f2afb",301:"0dadd2c9",613:"5efc9d3d",1317:"315aa7a9",1971:"da6513d5",2307:"6759b17e",2535:"814f3328",2655:"4cd22fa7",2659:"346551de",2708:"5ee0e852",2852:"b1a5869c",3089:"a6aa9e1f",3148:"8bb94aa1",3511:"63969280",3608:"9e4087bc",3922:"3eb12003",4013:"01a85c17",4059:"3aecf4c2",4163:"1d3fbc77",4189:"23f2110f",4195:"c4f5d8e4",4274:"5fc994c2",4354:"298e1cbf",4687:"646279b0",4885:"8e8f279c",5038:"e7e087cc",5782:"8d8b4977",5857:"d4f8d7b5",5968:"efe9c66f",6103:"ccc49370",6320:"3aa955b1",6430:"1dd8b324",6846:"319c539b",7338:"3488a21a",7485:"c7d749c3",7615:"2419ec42",7658:"c554d126",7695:"653f7965",7918:"17896441",8071:"73902fa9",8139:"012f7f96",8159:"bdc52102",8587:"bef1cd89",8610:"6875c492",8612:"f0ad3fbb",9387:"9494ffc1",9514:"1be78505",9518:"b48fcc4a",9697:"73fe69d8",9713:"470d070e",9743:"43040bd9",9817:"14eb3368"}[e]||e)+"."+{53:"3185821f",301:"8d3e3c29",613:"bbb76819",1317:"6f6b6428",1971:"627a1c50",2307:"086dea05",2535:"31a92c4f",2655:"dd2a0ecd",2659:"4f9e9fa3",2708:"0832cae0",2852:"96634377",3089:"6e015b54",3148:"b4d60c46",3511:"d33e5a36",3527:"e0b09d3a",3608:"53880c64",3922:"8d0599c7",4013:"fcb98749",4059:"f6e0718d",4163:"4d4ecafd",4189:"51c887a1",4195:"fa962420",4274:"7c232ad7",4354:"0a887d40",4687:"4f04f4cc",4885:"6f6b1a02",4972:"cbef3a9d",5038:"41b46196",5782:"61fc4b73",5857:"4b3cf04c",5968:"01595703",6048:"ccd22901",6103:"025441c7",6320:"d02aab99",6430:"e482eaed",6846:"e05e4044",7036:"ffea0361",7338:"63b3777e",7485:"ece2f88a",7615:"6efbedd4",7658:"f6eebe07",7695:"0598f099",7918:"5db64ed8",8071:"a59b2596",8139:"b4091524",8159:"1dfdd4c1",8587:"fac66822",8610:"d63cf4a1",8612:"ca0f83e6",9387:"e4a0bfbf",9514:"da5fda06",9518:"b2e8e1c1",9697:"f9812bca",9713:"29937d92",9743:"b0981966",9817:"19dc8723"}[e]+".js",b.miniCssF=e=>{},b.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),b.o=(e,c)=>Object.prototype.hasOwnProperty.call(e,c),d={},t="mithril-doc:",b.l=(e,c,a,f)=>{if(d[e])d[e].push(c);else{var r,o;if(void 0!==a)for(var n=document.getElementsByTagName("script"),i=0;i<n.length;i++){var l=n[i];if(l.getAttribute("src")==e||l.getAttribute("data-webpack")==t+a){r=l;break}}r||(o=!0,(r=document.createElement("script")).charset="utf-8",r.timeout=120,b.nc&&r.setAttribute("nonce",b.nc),r.setAttribute("data-webpack",t+a),r.src=e),d[e]=[c];var u=(c,a)=>{r.onerror=r.onload=null,clearTimeout(s);var t=d[e];if(delete d[e],r.parentNode&&r.parentNode.removeChild(r),t&&t.forEach((e=>e(a))),c)return c(a)},s=setTimeout(u.bind(null,void 0,{type:"timeout",target:r}),12e4);r.onerror=u.bind(null,r.onerror),r.onload=u.bind(null,r.onload),o&&document.head.appendChild(r)}},b.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},b.nmd=e=>(e.paths=[],e.children||(e.children=[]),e),b.p="/doc/",b.gca=function(e){return e={17896441:"7918",63969280:"3511","935f2afb":"53","0dadd2c9":"301","5efc9d3d":"613","315aa7a9":"1317",da6513d5:"1971","6759b17e":"2307","814f3328":"2535","4cd22fa7":"2655","346551de":"2659","5ee0e852":"2708",b1a5869c:"2852",a6aa9e1f:"3089","8bb94aa1":"3148","9e4087bc":"3608","3eb12003":"3922","01a85c17":"4013","3aecf4c2":"4059","1d3fbc77":"4163","23f2110f":"4189",c4f5d8e4:"4195","5fc994c2":"4274","298e1cbf":"4354","646279b0":"4687","8e8f279c":"4885",e7e087cc:"5038","8d8b4977":"5782",d4f8d7b5:"5857",efe9c66f:"5968",ccc49370:"6103","3aa955b1":"6320","1dd8b324":"6430","319c539b":"6846","3488a21a":"7338",c7d749c3:"7485","2419ec42":"7615",c554d126:"7658","653f7965":"7695","73902fa9":"8071","012f7f96":"8139",bdc52102:"8159",bef1cd89:"8587","6875c492":"8610",f0ad3fbb:"8612","9494ffc1":"9387","1be78505":"9514",b48fcc4a:"9518","73fe69d8":"9697","470d070e":"9713","43040bd9":"9743","14eb3368":"9817"}[e]||e,b.p+b.u(e)},(()=>{var e={1303:0,532:0};b.f.j=(c,a)=>{var d=b.o(e,c)?e[c]:void 0;if(0!==d)if(d)a.push(d[2]);else if(/^(1303|532)$/.test(c))e[c]=0;else{var t=new Promise(((a,t)=>d=e[c]=[a,t]));a.push(d[2]=t);var f=b.p+b.u(c),r=new Error;b.l(f,(a=>{if(b.o(e,c)&&(0!==(d=e[c])&&(e[c]=void 0),d)){var t=a&&("load"===a.type?"missing":a.type),f=a&&a.target&&a.target.src;r.message="Loading chunk "+c+" failed.\n("+t+": "+f+")",r.name="ChunkLoadError",r.type=t,r.request=f,d[1](r)}}),"chunk-"+c,c)}},b.O.j=c=>0===e[c];var c=(c,a)=>{var d,t,f=a[0],r=a[1],o=a[2],n=0;if(f.some((c=>0!==e[c]))){for(d in r)b.o(r,d)&&(b.m[d]=r[d]);if(o)var i=o(b)}for(c&&c(a);n<f.length;n++)t=f[n],b.o(e,t)&&e[t]&&e[t][0](),e[t]=0;return b.O(i)},a=self.webpackChunkmithril_doc=self.webpackChunkmithril_doc||[];a.forEach(c.bind(null,0)),a.push=c.bind(null,a.push.bind(a))})()})();