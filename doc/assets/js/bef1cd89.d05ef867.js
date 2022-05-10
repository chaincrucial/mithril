"use strict";(self.webpackChunkmithril_doc=self.webpackChunkmithril_doc||[]).push([[587],{3905:function(e,t,r){r.d(t,{Zo:function(){return c},kt:function(){return f}});var n=r(7294);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function l(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var s=n.createContext({}),u=function(e){var t=n.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},c=function(e){var t=u(e.components);return n.createElement(s.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},d=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,s=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),d=u(r),f=o,m=d["".concat(s,".").concat(f)]||d[f]||p[f]||a;return r?n.createElement(m,i(i({ref:t},c),{},{components:r})):n.createElement(m,i({ref:t},c))}));function f(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,i=new Array(a);i[0]=d;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l.mdxType="string"==typeof e?e:o,i[1]=l;for(var u=2;u<a;u++)i[u]=r[u];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}d.displayName="MDXCreateElement"},5089:function(e,t,r){r.r(t),r.d(t,{assets:function(){return c},contentTitle:function(){return s},default:function(){return f},frontMatter:function(){return l},metadata:function(){return u},toc:function(){return p}});var n=r(7462),o=r(3366),a=(r(7294),r(3905)),i=["components"],l={slug:2,title:"2. Use simple structured logging\n",authors:[],tags:["Draft"]},s=void 0,u={permalink:"/doc/adr/2",source:"@site/adr/002-use-structured-logging.md",title:"2. Use simple structured logging\n",description:"Status",date:"2022-05-10T12:04:00.000Z",formattedDate:"May 10, 2022",tags:[{label:"Draft",permalink:"/doc/adr/tags/draft"}],readingTime:.665,truncated:!1,authors:[],frontMatter:{slug:"2",title:"2. Use simple structured logging\n",authors:[],tags:["Draft"]},nextItem:{title:"1. Record Architecture Decisions\n",permalink:"/doc/adr/1"}},c={authorsImageUrls:[]},p=[{value:"Status",id:"status",level:2},{value:"Context",id:"context",level:2},{value:"Decision",id:"decision",level:2},{value:"Consequences",id:"consequences",level:2}],d={toc:p};function f(e){var t=e.components,r=(0,o.Z)(e,i);return(0,a.kt)("wrapper",(0,n.Z)({},d,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h2",{id:"status"},"Status"),(0,a.kt)("p",null,(0,a.kt)("strong",{parentName:"p"},"Draft")),(0,a.kt)("h2",{id:"context"},"Context"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Logs are a critical tool for operating any software system, enabling ",(0,a.kt)("a",{parentName:"li",href:"https://cloud.google.com/architecture/devops/devops-measurement-monitoring-and-observability"},"observability")," of the system."),(0,a.kt)("li",{parentName:"ul"},"Following ",(0,a.kt)("a",{parentName:"li",href:"https://12factor.net/logs"},"12 Factor Apps")," principles, providing the needed components and tools to be able to configure logging and monitoring should not be the responsibility of the software components")),(0,a.kt)("h2",{id:"decision"},"Decision"),(0,a.kt)("p",null,(0,a.kt)("em",{parentName:"p"},"Therefore")),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Each component of the system use ",(0,a.kt)("a",{parentName:"li",href:"https://www.sumologic.com/glossary/structured-logging/"},"Structured logging")," using documented and standardised JSON format for its logs"),(0,a.kt)("li",{parentName:"ul"},"Logs are always emitted to ",(0,a.kt)("inlineCode",{parentName:"li"},"stdout")," of the process the component is part of")),(0,a.kt)("h2",{id:"consequences"},"Consequences"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"The schema of the logged items should be properly documented in a JSON schema"),(0,a.kt)("li",{parentName:"ul"},"It is the responsibility of the node operator to consume the logs and process them"),(0,a.kt)("li",{parentName:"ul"},"We use existing libraries to provide needed log infrastructure, like ",(0,a.kt)("a",{parentName:"li",href:"https://zsiciarz.github.io/24daysofrust/book/vol2/day4.html"},"slog")," for Rust")))}f.isMDXComponent=!0}}]);