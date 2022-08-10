"use strict";(self.webpackChunkmithril_doc=self.webpackChunkmithril_doc||[]).push([[6430],{3905:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>m});var n=r(67294);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function c(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},o=Object.keys(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var l=n.createContext({}),s=function(e){var t=n.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},u=function(e){var t=s(e.components);return n.createElement(l.Provider,{value:t},e.children)},d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},p=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,o=e.originalType,l=e.parentName,u=c(e,["components","mdxType","originalType","parentName"]),p=s(r),m=a,f=p["".concat(l,".").concat(m)]||p[m]||d[m]||o;return r?n.createElement(f,i(i({ref:t},u),{},{components:r})):n.createElement(f,i({ref:t},u))}));function m(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=r.length,i=new Array(o);i[0]=p;var c={};for(var l in t)hasOwnProperty.call(t,l)&&(c[l]=t[l]);c.originalType=e,c.mdxType="string"==typeof e?e:a,i[1]=c;for(var s=2;s<o;s++)i[s]=r[s];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}p.displayName="MDXCreateElement"},88876:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>d,frontMatter:()=>o,metadata:()=>c,toc:()=>s});var n=r(87462),a=(r(67294),r(3905));const o={slug:1,title:"1. Record Architecture Decisions\n",authors:[],tags:["Accepted"]},i=void 0,c={permalink:"/doc/adr/1",source:"@site/adr/001-use-adr.md",title:"1. Record Architecture Decisions\n",description:"Status",date:"2022-08-10T13:32:48.000Z",formattedDate:"August 10, 2022",tags:[{label:"Accepted",permalink:"/doc/adr/tags/accepted"}],readingTime:.515,hasTruncateMarker:!1,authors:[],frontMatter:{slug:"1",title:"1. Record Architecture Decisions\n",authors:[],tags:["Accepted"]},prevItem:{title:"2. Use simple structured logging\n",permalink:"/doc/adr/2"}},l={authorsImageUrls:[]},s=[{value:"Status",id:"status",level:2},{value:"Context",id:"context",level:2},{value:"Decision",id:"decision",level:2},{value:"Consequences",id:"consequences",level:2}],u={toc:s};function d(e){let{components:t,...r}=e;return(0,a.kt)("wrapper",(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h2",{id:"status"},"Status"),(0,a.kt)("p",null,"Accepted"),(0,a.kt)("h2",{id:"context"},"Context"),(0,a.kt)("p",null,"We are in search for a means to describe our technical architecture."),(0,a.kt)("p",null,"We are a small team working in a very lean and agile way (XP), so we naturally\nprefer also light-weight documentation methods which also accomodate change\neasily."),(0,a.kt)("h2",{id:"decision"},"Decision"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"We will use ",(0,a.kt)("em",{parentName:"li"},"Architecture Decision Records"),", as described by Michael Nygard in\nthis\n",(0,a.kt)("a",{parentName:"li",href:"http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions"},"article"),"."),(0,a.kt)("li",{parentName:"ul"},"We will follow the convention of storing those ADRs as Markdown formatted\ndocuments stored under ",(0,a.kt)("inlineCode",{parentName:"li"},"docs/adr")," directory, as exemplified in Nat Pryce's\n",(0,a.kt)("a",{parentName:"li",href:"https://github.com/npryce/adr-tools"},"adr-tools"),". This does not imply we will\nbe using ",(0,a.kt)("inlineCode",{parentName:"li"},"adr-tools")," itself.")),(0,a.kt)("h2",{id:"consequences"},"Consequences"),(0,a.kt)("p",null,"See Michael Nygard's article, linked above."))}d.isMDXComponent=!0}}]);