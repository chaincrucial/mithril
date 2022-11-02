"use strict";(self.webpackChunkmithril_doc=self.webpackChunkmithril_doc||[]).push([[9356],{3905:(e,t,n)=>{n.d(t,{Zo:()=>m,kt:()=>c});var r=n(7294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var p=r.createContext({}),s=function(e){var t=r.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},m=function(e){var t=s(e.components);return r.createElement(p.Provider,{value:t},e.children)},d={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},u=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,i=e.originalType,p=e.parentName,m=l(e,["components","mdxType","originalType","parentName"]),u=s(n),c=a,h=u["".concat(p,".").concat(c)]||u[c]||d[c]||i;return n?r.createElement(h,o(o({ref:t},m),{},{components:n})):r.createElement(h,o({ref:t},m))}));function c(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=n.length,o=new Array(i);o[0]=u;var l={};for(var p in t)hasOwnProperty.call(t,p)&&(l[p]=t[p]);l.originalType=e,l.mdxType="string"==typeof e?e:a,o[1]=l;for(var s=2;s<i;s++)o[s]=n[s];return r.createElement.apply(null,o)}return r.createElement.apply(null,n)}u.displayName="MDXCreateElement"},4152:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>o,default:()=>d,frontMatter:()=>i,metadata:()=>l,toc:()=>s});var r=n(7462),a=(n(7294),n(3905));const i={title:"Mithril environments are updated",authors:[{name:"Mithril Team"}],tags:["release-process","re-spin","preview","preprod","environments"]},o=void 0,l={permalink:"/doc/dev-blog/2022/10/28/updated-environments",source:"@site/blog/2022-10-28-updated-environments.md",title:"Mithril environments are updated",description:"The Mithril environments are updated",date:"2022-10-28T00:00:00.000Z",formattedDate:"October 28, 2022",tags:[{label:"release-process",permalink:"/doc/dev-blog/tags/release-process"},{label:"re-spin",permalink:"/doc/dev-blog/tags/re-spin"},{label:"preview",permalink:"/doc/dev-blog/tags/preview"},{label:"preprod",permalink:"/doc/dev-blog/tags/preprod"},{label:"environments",permalink:"/doc/dev-blog/tags/environments"}],readingTime:1.3,hasTruncateMarker:!1,authors:[{name:"Mithril Team"}],frontMatter:{title:"Mithril environments are updated",authors:[{name:"Mithril Team"}],tags:["release-process","re-spin","preview","preprod","environments"]},nextItem:{title:"Mithril Keys Certification",permalink:"/doc/dev-blog/2022/10/11/keys-certification-badge"}},p={authorsImageUrls:[void 0]},s=[{value:"The Mithril environments are updated",id:"the-mithril-environments-are-updated",level:3}],m={toc:s};function d(e){let{components:t,...n}=e;return(0,a.kt)("wrapper",(0,r.Z)({},m,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h3",{id:"the-mithril-environments-are-updated"},"The Mithril environments are updated"),(0,a.kt)("p",null,(0,a.kt)("strong",{parentName:"p"},"PR"),": ",(0,a.kt)("inlineCode",{parentName:"p"},"New hosted environments")," ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/input-output-hk/mithril/pull/561"},"#561")),(0,a.kt)("p",null,(0,a.kt)("strong",{parentName:"p"},"Issue"),": ",(0,a.kt)("inlineCode",{parentName:"p"},"Setup new hosted environments for testing-preview, pre-release-preview and release-preprod) with their terraform and GitHub environments")," ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/input-output-hk/mithril/issues/542"},"#542")),(0,a.kt)("p",null,"On Tuesday, November 1st, 2022 the ",(0,a.kt)("inlineCode",{parentName:"p"},"preview")," Cardano network will be re-spun and will be unavailable for 48h."),(0,a.kt)("p",null,"In the mean time, the Mitril team is also implementing a new Release Process that will make use of several new environments."),(0,a.kt)("p",null,"The Mithril testing environments are thus evolving in this context:"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"The current testing environment that runs on ",(0,a.kt)("inlineCode",{parentName:"p"},"preview")," network and that most of the Pioneer SPOs are running is ",(0,a.kt)("strong",{parentName:"p"},"deprecated")," and will be decommissioned just after the ",(0,a.kt)("inlineCode",{parentName:"p"},"preview")," network re-spin.")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"This environment will then be replaced by a new ",(0,a.kt)("inlineCode",{parentName:"p"},"pre-release-preview")," environment open to SPOs that are eager to test pre releases of the Mithril nodes.")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"A new ",(0,a.kt)("inlineCode",{parentName:"p"},"release-preprod")," environment has been launched on the ",(0,a.kt)("inlineCode",{parentName:"p"},"preprod")," Cardano nework and will become the ",(0,a.kt)("inlineCode",{parentName:"p"},"stable")," environment on which SPOs are encouraged to run their nodes.")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"\u26a0\ufe0f The new ",(0,a.kt)("inlineCode",{parentName:"p"},"release-preprod")," environment is in ",(0,a.kt)("inlineCode",{parentName:"p"},"unstable")," status, therefore it is subject to re-genesis. We expect it to be in ",(0,a.kt)("inlineCode",{parentName:"p"},"stable")," status within 1-2 weeks."))),(0,a.kt)("p",null,"In the future, when Mithril reaches ",(0,a.kt)("inlineCode",{parentName:"p"},"mainnet"),", we assume that the ",(0,a.kt)("inlineCode",{parentName:"p"},"release-preprod")," will be replaced by a ",(0,a.kt)("inlineCode",{parentName:"p"},"release-mainnet")," environment. This means that we will have the following environments at this time: ",(0,a.kt)("inlineCode",{parentName:"p"},"testing-preview"),", ",(0,a.kt)("inlineCode",{parentName:"p"},"pre-release-preprod")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"release-mainnet"),"."),(0,a.kt)("p",null,"More information about:"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"The ",(0,a.kt)("inlineCode",{parentName:"p"},"Mithril Networks")," and their availability ",(0,a.kt)("a",{parentName:"p",href:"https://mithril.network/doc/manual/developer-docs/references#mithril-networks"},"here"),".")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("p",{parentName:"li"},"The ",(0,a.kt)("inlineCode",{parentName:"p"},"Release Process")," is available in this ",(0,a.kt)("a",{parentName:"p",href:"https://mithril.network/doc/adr/3"},"ADR"),"."))),(0,a.kt)("p",null,"Feel free to reach out to us on the ",(0,a.kt)("a",{parentName:"p",href:"https://discord.gg/5kaErDKDRq"},"Discord channel")," for questions and/or help."))}d.isMDXComponent=!0}}]);