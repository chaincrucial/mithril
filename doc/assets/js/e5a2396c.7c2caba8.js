"use strict";(self.webpackChunkmithril_doc=self.webpackChunkmithril_doc||[]).push([[7119],{3905:(e,t,n)=>{n.d(t,{Zo:()=>d,kt:()=>u});var r=n(67294);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,i=function(e,t){if(null==e)return{};var n,r,i={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var c=r.createContext({}),s=function(e){var t=r.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},d=function(e){var t=s(e.components);return r.createElement(c.Provider,{value:t},e.children)},p="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},h=r.forwardRef((function(e,t){var n=e.components,i=e.mdxType,a=e.originalType,c=e.parentName,d=l(e,["components","mdxType","originalType","parentName"]),p=s(n),h=i,u=p["".concat(c,".").concat(h)]||p[h]||m[h]||a;return n?r.createElement(u,o(o({ref:t},d),{},{components:n})):r.createElement(u,o({ref:t},d))}));function u(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var a=n.length,o=new Array(a);o[0]=h;var l={};for(var c in t)hasOwnProperty.call(t,c)&&(l[c]=t[c]);l.originalType=e,l[p]="string"==typeof e?e:i,o[1]=l;for(var s=2;s<a;s++)o[s]=n[s];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}h.displayName="MDXCreateElement"},1142:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>o,default:()=>m,frontMatter:()=>a,metadata:()=>l,toc:()=>s});var r=n(87462),i=(n(67294),n(3905));const a={title:"Mithril client has got a brand new interface",authors:[{name:"Mithril Team"}],tags:["client","certificate","mithril-stake-distribution"]},o=void 0,l={permalink:"/doc/dev-blog/2023/06/14/new-client-interface",source:"@site/blog/2023-06-14-new-client-interface.md",title:"Mithril client has got a brand new interface",description:"Mithril client interface is evolving",date:"2023-06-14T00:00:00.000Z",formattedDate:"June 14, 2023",tags:[{label:"client",permalink:"/doc/dev-blog/tags/client"},{label:"certificate",permalink:"/doc/dev-blog/tags/certificate"},{label:"mithril-stake-distribution",permalink:"/doc/dev-blog/tags/mithril-stake-distribution"}],readingTime:1.155,hasTruncateMarker:!1,authors:[{name:"Mithril Team"}],frontMatter:{title:"Mithril client has got a brand new interface",authors:[{name:"Mithril Team"}],tags:["client","certificate","mithril-stake-distribution"]},nextItem:{title:"Mithril Era Switch",permalink:"/doc/dev-blog/2023/03/02/era-switch-feature"}},c={authorsImageUrls:[void 0]},s=[{value:"Mithril client interface is evolving",id:"mithril-client-interface-is-evolving",level:3}],d={toc:s},p="wrapper";function m(e){let{components:t,...n}=e;return(0,i.kt)(p,(0,r.Z)({},d,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h3",{id:"mithril-client-interface-is-evolving"},"Mithril client interface is evolving"),(0,i.kt)("p",null,"For the last few months, we have implemented the capability for the Mithril protocol to sign multiple types of data: on top of the already existing Cardano node database snapshots, the Mithril stake distribution is now also signed on its own.\nIn order to make the client able to work on the different types of data that are certified, we have changed its command line API.\nFor example:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre"},"$> ./mithril-client list\n")),(0,i.kt)("p",null,"This command was previously used to list Cardano node snapshots. It has been abandoned in favor of a more explicit syntax:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre"},"$> ./mithril-client snapshot list\n")),(0,i.kt)("p",null,"Furthermore, the old version had two different subcommands to 1. download and 2. verify a snapshot. These 2 commands have now be merged into one single ",(0,i.kt)("inlineCode",{parentName:"p"},"download")," command:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre"},"$> ./mithril-client snapshot download  5109c1eaa6619bc\u2026\n")),(0,i.kt)("p",null,"This organization of the client opens the use of a new ",(0,i.kt)("inlineCode",{parentName:"p"},"mithril-stake-distribution")," sub-command:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre"},"$> ./mithril-client mithril-stake-distribution list\n")),(0,i.kt)("p",null,"Which can be aliased into a handy"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre"},"$> ./mithril-client msd list\n")),(0,i.kt)("p",null,"As for the Cardano snapshots, it is possible to download and verify the stake distribution involved in Mithril multi-signatures as a JSON file:"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre"},"$> ./mithril-client msd download 713e2803e998f\u2026\n")),(0,i.kt)("p",null,"If the file certification can be verified, it is saved on the disk."),(0,i.kt)("p",null,"Feel free to reach out to us on the ",(0,i.kt)("a",{parentName:"p",href:"https://discord.gg/5kaErDKDRq"},"Discord channel")," for questions and/or help."))}m.isMDXComponent=!0}}]);