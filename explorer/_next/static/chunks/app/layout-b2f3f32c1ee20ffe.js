(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[185],{6308:function(e,t,r){Promise.resolve().then(r.bind(r,9978)),Promise.resolve().then(r.t.bind(r,3222,23)),Promise.resolve().then(r.t.bind(r,6685,23)),Promise.resolve().then(r.t.bind(r,3453,23)),Promise.resolve().then(r.t.bind(r,1649,23)),Promise.resolve().then(r.t.bind(r,2112,23)),Promise.resolve().then(r.t.bind(r,810,23))},4402:function(e,t,r){"use strict";t.Z=["https://aggregator.release-mainnet.api.mithril.network/aggregator","https://aggregator.release-preprod.api.mithril.network/aggregator","https://aggregator.pre-release-preview.api.mithril.network/aggregator","https://aggregator.testing-preview.api.mithril.network/aggregator","http://localhost:8080/aggregator"]},7153:function(e,t,r){"use strict";r.d(t,{W:function(){return a}});let a="aggregator"},9978:function(e,t,r){"use strict";r.r(t),r.d(t,{Providers:function(){return b}});var a=r(7437),o=r(7153),n=r(64),l=r(6641),g=r(4402),i=r(3956);let s="Explorer_State",u=e=>{var t;return(0,n.xC)({reducer:{settings:l.xj.reducer},preloadedState:{...t=function(){if(localStorage){let e=localStorage.getItem(s);return e?JSON.parse(e):void 0}}(),settings:function(e,t){var r,a;let o,n=null!=e?e:l.E3,s=(r=n.availableAggregators,a=g.Z,o=r.filter(e=>!a.includes(e)),[...a,...o]);return t&&(0,i.checkUrl)(t)?(s.includes(t)||s.push(t),n={...n,selectedAggregator:t,availableAggregators:s,canRemoveSelected:!g.Z.includes(t)}):n={...n,availableAggregators:s},n}(null==t?void 0:t.settings,e)}})};var c=r(3198),d=r(4033),f=r(2265);function b(e){let{children:t}=e,r=(0,d.useSearchParams)(),n=r.get(o.W),[l,g]=(0,f.useState)(u(n));return l.subscribe(()=>{var e;return e=l.getState(),void(localStorage&&localStorage.setItem(s,JSON.stringify(e)))}),(0,a.jsx)(c.zt,{store:l,children:t})}},6641:function(e,t,r){"use strict";r.d(t,{E3:function(){return l},JV:function(){return i},OR:function(){return c},VT:function(){return u},k6:function(){return d},uI:function(){return s},xj:function(){return g}});var a=r(64),o=r(4402),n=r(3956);let l={autoUpdate:!0,updateInterval:1e4,selectedAggregator:o.Z[0],availableAggregators:o.Z,canRemoveSelected:!1},g=(0,a.oM)({name:"settings",initialState:l,reducers:{setUpdateInterval:(e,t)=>{e.updateInterval=t.payload},toggleAutoUpdate:e=>{e.autoUpdate=!e.autoUpdate},selectAggregator:(e,t)=>{if(!(0,n.checkUrl)(t.payload))return e;let r=e.availableAggregators.includes(t.payload)?e.availableAggregators:[...e.availableAggregators,t.payload];return{...e,selectedAggregator:t.payload,availableAggregators:r,canRemoveSelected:!o.Z.includes(t.payload)}},removeSelectedAggregator:e=>o.Z.includes(e.selectedAggregator)?e:{...e,selectedAggregator:e.availableAggregators.at(0),availableAggregators:e.availableAggregators.filter(t=>t!==e.selectedAggregator),canRemoveSelected:!o.Z.includes(e.availableAggregators.at(0))}}}),{setUpdateInterval:i,toggleAutoUpdate:s,selectAggregator:u,removeSelectedAggregator:c}=g.actions,d=e=>e.settings.selectedAggregator;g.reducer},3956:function(e){"use strict";let t=e=>e/1e6,r=function(e){let t=arguments.length>1&&void 0!==arguments[1]?arguments[1]:2;return e.toLocaleString(void 0,{maximumFractionDigits:t})};e.exports={checkUrl:function(e){try{return new URL(e),!0}catch(e){return!1}},formatStake:function(e){let a=t(e),o=[{suffix:"B",value:1e9},{suffix:"M",value:1e6},{suffix:"K",value:1e3},{suffix:"",value:1}].find(e=>Math.abs(a)>=e.value-.001);return o?"".concat(r(a/o.value)).concat(o.suffix,"₳"):"".concat(r(a),"₳")},setChartJsDefaults:function(e){let t=["rgba(255, 99, 132, 0.2)","rgba(255, 159, 64, 0.2)","rgba(255, 205, 86, 0.2)","rgba(75, 192, 192, 0.2)","rgba(54, 162, 235, 0.2)","rgba(153, 102, 255, 0.2)","rgba(201, 203, 207, 0.2)"],r=["rgb(255, 99, 132)","rgb(255, 159, 64)","rgb(255, 205, 86)","rgb(75, 192, 192)","rgb(54, 162, 235)","rgb(153, 102, 255)","rgb(201, 203, 207)"];e.defaults.plugins.legend.display=!1,e.defaults.elements.arc.backgroundColor=t,e.defaults.elements.arc.borderColor=r,e.defaults.elements.arc.borderWidth=1,e.defaults.elements.bar.backgroundColor=t,e.defaults.elements.bar.borderColor=r,e.defaults.elements.bar.borderWidth=1},toAda:t,formatCurrency:r}},1649:function(){},3453:function(e){e.exports={container:"explorer_container__e4y3J",main:"explorer_main__72BOO",footer:"explorer_footer__NDYaK",title:"explorer_title__4AQZM",code:"explorer_code__d9zj2",logo:"explorer_logo__qsx9l"}}},function(e){e.O(0,[129,807,141,282,971,596,744],function(){return e(e.s=6308)}),_N_E=e.O()}]);