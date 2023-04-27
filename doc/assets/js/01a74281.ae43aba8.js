"use strict";(self.webpackChunkmithril_doc=self.webpackChunkmithril_doc||[]).push([[47],{3905:(e,t,i)=>{i.d(t,{Zo:()=>d,kt:()=>u});var r=i(67294);function n(e,t,i){return t in e?Object.defineProperty(e,t,{value:i,enumerable:!0,configurable:!0,writable:!0}):e[t]=i,e}function a(e,t){var i=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),i.push.apply(i,r)}return i}function o(e){for(var t=1;t<arguments.length;t++){var i=null!=arguments[t]?arguments[t]:{};t%2?a(Object(i),!0).forEach((function(t){n(e,t,i[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(i)):a(Object(i)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(i,t))}))}return e}function l(e,t){if(null==e)return{};var i,r,n=function(e,t){if(null==e)return{};var i,r,n={},a=Object.keys(e);for(r=0;r<a.length;r++)i=a[r],t.indexOf(i)>=0||(n[i]=e[i]);return n}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)i=a[r],t.indexOf(i)>=0||Object.prototype.propertyIsEnumerable.call(e,i)&&(n[i]=e[i])}return n}var s=r.createContext({}),p=function(e){var t=r.useContext(s),i=t;return e&&(i="function"==typeof e?e(t):o(o({},t),e)),i},d=function(e){var t=p(e.components);return r.createElement(s.Provider,{value:t},e.children)},h="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},c=r.forwardRef((function(e,t){var i=e.components,n=e.mdxType,a=e.originalType,s=e.parentName,d=l(e,["components","mdxType","originalType","parentName"]),h=p(i),c=n,u=h["".concat(s,".").concat(c)]||h[c]||m[c]||a;return i?r.createElement(u,o(o({ref:t},d),{},{components:i})):r.createElement(u,o({ref:t},d))}));function u(e,t){var i=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var a=i.length,o=new Array(a);o[0]=c;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[h]="string"==typeof e?e:n,o[1]=l;for(var p=2;p<a;p++)o[p]=i[p];return r.createElement.apply(null,o)}return r.createElement.apply(null,i)}c.displayName="MDXCreateElement"},31182:(e,t,i)=>{i.r(t),i.d(t,{assets:()=>s,contentTitle:()=>o,default:()=>m,frontMatter:()=>a,metadata:()=>l,toc:()=>p});var r=i(87462),n=(i(67294),i(3905));const a={title:"Mithril Keys Certification",authors:[{name:"Mithril Team"}],tags:["cardano","poolId","operational-certificate","kes-keys","mithril-keys","hybrid-mode"]},o=void 0,l={permalink:"/doc/dev-blog/2022/10/11/keys-certification-badge",source:"@site/blog/2022-10-11-keys-certification-badge/index.md",title:"Mithril Keys Certification",description:"Update 2022/12/19: The signer registration with declarative PoolId has been decommissioned.",date:"2022-10-11T00:00:00.000Z",formattedDate:"October 11, 2022",tags:[{label:"cardano",permalink:"/doc/dev-blog/tags/cardano"},{label:"poolId",permalink:"/doc/dev-blog/tags/pool-id"},{label:"operational-certificate",permalink:"/doc/dev-blog/tags/operational-certificate"},{label:"kes-keys",permalink:"/doc/dev-blog/tags/kes-keys"},{label:"mithril-keys",permalink:"/doc/dev-blog/tags/mithril-keys"},{label:"hybrid-mode",permalink:"/doc/dev-blog/tags/hybrid-mode"}],readingTime:2.39,hasTruncateMarker:!1,authors:[{name:"Mithril Team"}],frontMatter:{title:"Mithril Keys Certification",authors:[{name:"Mithril Team"}],tags:["cardano","poolId","operational-certificate","kes-keys","mithril-keys","hybrid-mode"]},prevItem:{title:"Mithril environments are updated",permalink:"/doc/dev-blog/2022/10/28/updated-environments"},nextItem:{title:"Mithril internal stores switch to SQLite.",permalink:"/doc/dev-blog/2022/09/14/sqlite-stores"}},s={authorsImageUrls:[void 0]},p=[{value:"The way the Mithril nodes handle the Certification of the SPOs is evolving",id:"the-way-the-mithril-nodes-handle-the-certification-of-the-spos-is-evolving",level:3},{value:"Upgrade a Mithril Signer running a previous version",id:"upgrade-a-mithril-signer-running-a-previous-version",level:4},{value:"Hybrid Certification mode in the Mithril network",id:"hybrid-certification-mode-in-the-mithril-network",level:4},{value:"How Keys Certification works",id:"how-keys-certification-works",level:4}],d={toc:p},h="wrapper";function m(e){let{components:t,...a}=e;return(0,n.kt)(h,(0,r.Z)({},d,a,{components:t,mdxType:"MDXLayout"}),(0,n.kt)("p",null,(0,n.kt)("strong",{parentName:"p"},"Update 2022/12/19"),": The signer registration with ",(0,n.kt)("strong",{parentName:"p"},"declarative")," PoolId has been decommissioned."),(0,n.kt)("p",null,(0,n.kt)("strong",{parentName:"p"},"Update 2022/11/30"),": The signer registration with ",(0,n.kt)("strong",{parentName:"p"},"declarative")," PoolId has been deprecated and the ",(0,n.kt)("strong",{parentName:"p"},"certified")," PoolId is now the stable mode."),(0,n.kt)("h3",{id:"the-way-the-mithril-nodes-handle-the-certification-of-the-spos-is-evolving"},"The way the Mithril nodes handle the Certification of the SPOs is evolving"),(0,n.kt)("p",null,(0,n.kt)("strong",{parentName:"p"},"PR"),": ",(0,n.kt)("inlineCode",{parentName:"p"},"New STM registration procedure")," ",(0,n.kt)("a",{parentName:"p",href:"https://github.com/input-output-hk/mithril/pull/433"},"#433")),(0,n.kt)("p",null,(0,n.kt)("strong",{parentName:"p"},"Issues"),": ",(0,n.kt)("inlineCode",{parentName:"p"},"Implement Certification of the Mithril Verification Keys in Signer/Aggregator")," ",(0,n.kt)("a",{parentName:"p",href:"https://github.com/input-output-hk/mithril/issues/455"},"#455")),(0,n.kt)("p",null,"We have released a new Mithril Signer Verification Keys Certification mechanism:"),(0,n.kt)("ul",null,(0,n.kt)("li",{parentName:"ul"},"Mithril Signer nodes running the previous version are still able to interact with the network without any further intervention"),(0,n.kt)("li",{parentName:"ul"},"Mithril Signer nodes that are updated from a previous version must migrate some of their stores"),(0,n.kt)("li",{parentName:"ul"},"This mechanism is ",(0,n.kt)("strong",{parentName:"li"},"experimental")," and can be activated on demand by the SPOs")),(0,n.kt)("h4",{id:"upgrade-a-mithril-signer-running-a-previous-version"},"Upgrade a Mithril Signer running a previous version"),(0,n.kt)("p",null,"The SPOs need to recompile their Signer node (as in this ",(0,n.kt)("a",{parentName:"p",href:"https://mithril.network/doc/manual/getting-started/run-signer-node"},"guide"),")."),(0,n.kt)("p",null,"The data stores of the node need to be updated by running the following command:"),(0,n.kt)("pre",null,(0,n.kt)("code",{parentName:"pre",className:"language-bash"},"# The path to your data stores directory, which defaults to:\nDATA_STORES_DIRECTORY=/opt/mithril/mithril-signer/stores\n\n# Run this command to upgrade your stores:\nsqlite3 ${DATA_STORES_DIRECTORY}/signer.sqlite3 \"UPDATE protocol_initializer SET value = json_object('stm_initializer', json(value), 'kes_signature', null) WHERE json_extract(value, '$.stm_initializer') IS NULL;\"\n")),(0,n.kt)("p",null,"\u26a0\ufe0f If you don't update your data stores with this procedure, your node will not be able to register to the Aggregator temporarily. It should then take up to ",(0,n.kt)("inlineCode",{parentName:"p"},"3")," epochs before it is able to successfully register its individual signatures with the Aggregator."),(0,n.kt)("h4",{id:"hybrid-certification-mode-in-the-mithril-network"},"Hybrid Certification mode in the Mithril network"),(0,n.kt)("p",null,"From now, SPOs can either run their node by:"),(0,n.kt)("ul",null,(0,n.kt)("li",{parentName:"ul"},(0,n.kt)("p",{parentName:"li"},(0,n.kt)("strong",{parentName:"p"},"Declaring their Cardano ",(0,n.kt)("inlineCode",{parentName:"strong"},"PoolId")),":"),(0,n.kt)("ul",{parentName:"li"},(0,n.kt)("li",{parentName:"ul"},"This is the mode that all nodes were running prior to this release"),(0,n.kt)("li",{parentName:"ul"},"This mode is still the ",(0,n.kt)("strong",{parentName:"li"},"stable")," mode"),(0,n.kt)("li",{parentName:"ul"},"We intend to deprecate this mode in the near future"))),(0,n.kt)("li",{parentName:"ul"},(0,n.kt)("p",{parentName:"li"},(0,n.kt)("strong",{parentName:"p"},"Certifying their Cardano ",(0,n.kt)("inlineCode",{parentName:"strong"},"PoolId")),":"),(0,n.kt)("ul",{parentName:"li"},(0,n.kt)("li",{parentName:"ul"},"The certification is done by providing the Mithril Signer node with ",(0,n.kt)("inlineCode",{parentName:"li"},"KES Secret Key Path")," and ",(0,n.kt)("inlineCode",{parentName:"li"},"Operational Certificate Path")),(0,n.kt)("li",{parentName:"ul"},"This is an ",(0,n.kt)("strong",{parentName:"li"},"experimental")," mode"),(0,n.kt)("li",{parentName:"ul"},"We intend to make this mode the only way of providing a ",(0,n.kt)("inlineCode",{parentName:"li"},"PoolId")," in the near future"),(0,n.kt)("li",{parentName:"ul"},"These ",(0,n.kt)("inlineCode",{parentName:"li"},"PoolIds")," will be marked with a ",(0,n.kt)("inlineCode",{parentName:"li"},"Verified Signer")," green badge in the ",(0,n.kt)("a",{parentName:"li",href:"https://mithril.network/explorer/"},"Mithril Explorer")," (",(0,n.kt)("inlineCode",{parentName:"li"},"2")," epochs after activating the Certification mode)")))),(0,n.kt)("p",null,"The setup of a Mithril Signer node with these two modes is available in this ",(0,n.kt)("a",{parentName:"p",href:"https://mithril.network/doc/manual/getting-started/run-signer-node"},"guide"),"."),(0,n.kt)("p",null,"Here is an example of the ",(0,n.kt)("inlineCode",{parentName:"p"},"Verified Signer")," badge displayed in the Certificate details popin:\n",(0,n.kt)("img",{alt:"Verified Signer Badge",src:i(74190).Z,width:"550",height:"221"})),(0,n.kt)("h4",{id:"how-keys-certification-works"},"How Keys Certification works"),(0,n.kt)("p",null,"We rely on the Cardano ",(0,n.kt)("inlineCode",{parentName:"p"},"KES Keys")," and ",(0,n.kt)("inlineCode",{parentName:"p"},"Operational Certificate")," to be able to:"),(0,n.kt)("ul",null,(0,n.kt)("li",{parentName:"ul"},"Compute automatically the ",(0,n.kt)("inlineCode",{parentName:"li"},"PoolId")," from a valid ",(0,n.kt)("inlineCode",{parentName:"li"},"Operational Certificate")),(0,n.kt)("li",{parentName:"ul"},"Sign the ",(0,n.kt)("inlineCode",{parentName:"li"},"Mithril Signer Verification Key")," with the ",(0,n.kt)("inlineCode",{parentName:"li"},"KES Secret Key")),(0,n.kt)("li",{parentName:"ul"},"Verify that the ",(0,n.kt)("inlineCode",{parentName:"li"},"Mithril Signer Verification Key")," is associated to the owner of the pool")),(0,n.kt)("p",null,(0,n.kt)("img",{alt:"Keys Certification Schema",src:i(13409).Z,width:"1134",height:"881"})),(0,n.kt)("p",null,"Feel free to reach out to us on the ",(0,n.kt)("a",{parentName:"p",href:"https://discord.gg/5kaErDKDRq"},"Discord channel")," for questions and/or help."))}m.isMDXComponent=!0},74190:(e,t,i)=>{i.d(t,{Z:()=>r});const r=i.p+"assets/images/badge-d830657d3818b56eb6d9dd154085e753.png"},13409:(e,t,i)=>{i.d(t,{Z:()=>r});const r=i.p+"assets/images/schema-2bb1f4c4f967eddf4006f3acb8dbcb88.jpg"}}]);