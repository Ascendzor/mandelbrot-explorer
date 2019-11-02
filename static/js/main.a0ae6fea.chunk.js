(this.webpackJsonpmandelbrotexplorer=this.webpackJsonpmandelbrotexplorer||[]).push([[0],{100:function(e,t,n){},205:function(e,t,n){"use strict";n.r(t);var o=n(0),a=n.n(o),r=n(26),i=n.n(r),c=(n(99),n(100),n(27)),l=n(206),s=n(13),u=n.n(s),m=n(80),d=n(81),p=n(91),h=n(82),f=n(92),g=n(207),b=n(40),w=n(101),E=function(e){return[e.x,e.y,e.z].join(" ")};console.log(Object({NODE_ENV:"production",PUBLIC_URL:"/mandelbrot-explorer"}));var v=Array.from({length:4}).map((function(e){return new Worker("/mandelbrot-explorer/workers.js")}));v.forEach((function(e){e.onmessage=function(e){w.publish("onTileLoad",e.data)}}));var y=0;u.a.MandelbrotLayer=u.a.GridLayer.extend({createTile:function(e,t){var n=document.createElement("canvas");return n.width=n.height=256,function(e){var t=e.coords,n=e.computeOption;return new Promise((function(e,o){w.subscribe("onTileLoad",(function(n,o){if(E(t)===E(o.coords))return e(o.imageData)})),v[y].postMessage({coords:t,computeOption:n}),y=(y+1)%v.length}))}({coords:e,computeOption:"rust"}).then((function(e){n.getContext("2d").putImageData(e,0,0),t(null,n)})),n}});var O=function(e){function t(){return Object(m.a)(this,t),Object(p.a)(this,Object(h.a)(t).apply(this,arguments))}return Object(f.a)(t,e),Object(d.a)(t,[{key:"createLeafletElement",value:function(e){return new u.a.MandelbrotLayer(e)}}]),t}(g.a),k=Object(b.b)(O),j=n(23),C=function(e){var t=e.onClick;return a.a.createElement("div",{className:"Share mapButton",onClick:t,style:{width:80,height:80,borderRadius:10,padding:5,paddingTop:10,fontSize:12,color:"white"}},"Share",a.a.createElement(j.b,{style:{paddingTop:6,width:50,height:50}}))},S=function(e){var t=e.onClick;return a.a.createElement("div",{className:"Share mapButton",onClick:t,style:{width:40,height:40,borderRadius:10,padding:10,paddingTop:6,fontSize:12,color:"white"}},"Donate",a.a.createElement(j.a,{style:{paddingTop:1,width:30,height:30}}))},x=function(e){var t=e.onClick;return a.a.createElement("div",{className:"Information mapButton",onClick:t,style:{width:60,height:60,borderRadius:10,padding:5,paddingTop:5,fontSize:12,color:"white"}},"Information",a.a.createElement(j.c,{style:{paddingTop:5,width:40,height:40}}))},T=n(61),N=n.n(T),z=n(87),A=n.n(z),P=n(85),D=n.n(P),R=function(e){var t=e.onClose;return a.a.createElement("div",{className:"InformationBox",style:{width:400,borderRadius:10,padding:10,paddingBottom:20,fontSize:16,color:"white",textAlign:"center",background:"rgba(40, 40, 40, .94)",display:"relative"}},a.a.createElement("div",null,a.a.createElement("p",null,"Welcome"),a.a.createElement("p",null,"This is a map of the Mandelbrot Set. You can interact with this map like you do with Google Maps."),a.a.createElement("p",null,"Pinch, zoom, scroll, drag all work."),a.a.createElement("hr",null),a.a.createElement("p",null,"None of the imagery you see here was man-made."),a.a.createElement("p",null,"The mathematical formula that generated this is:"),a.a.createElement("p",null,"Z = Z \xb2 + C"),a.a.createElement("hr",null),a.a.createElement("p",null,"How should you use this?"),a.a.createElement("p",{style:{fontWeight:"bold",fontSize:18}},a.a.createElement("span",{style:{color:"#4A9FF4"}},"Explore "),a.a.createElement("span",{style:{color:"#54AA00"}},"Appreciate "),a.a.createElement("span",{style:{color:"#B20C54"}},"Share")),a.a.createElement("hr",null),a.a.createElement("p",null,"Here are resources where you can learn more"),a.a.createElement("p",null,a.a.createElement("a",{className:"link",target:"_blank",href:"https://www.youtube.com/watch?v=NGMRB4O922I"},"How it is calculated mathematically")),a.a.createElement("p",null,a.a.createElement("a",{className:"link",target:"_blank",href:"https://www.youtube.com/watch?v=DKHucotq6J0&feature=youtu.be&t=69"},"Humourous religious description"))),a.a.createElement(D.a,{onClick:t,style:{position:"absolute",top:0,right:0}},a.a.createElement(j.d,{style:{color:"rgba(230, 230, 230, .5)"}})))},B=[[-14336,-8192],[14336,8192]],I=function(){var e=Object(o.useState)({center:[0,-256],zoom:0}),t=Object(c.a)(e,2),n=t[0],r=t[1],i=Object(o.useState)(!1),s=Object(c.a)(i,2),m=s[0],d=s[1],p=Object(o.useState)(""),h=Object(c.a)(p,2),f=h[0],g=h[1],b=Object(o.useState)(0),w=Object(c.a)(b,2),E=w[0],v=w[1],y=Object(o.useState)(!0),O=Object(c.a)(y,2),j=O[0],T=O[1];return Object(o.useEffect)((function(){var e=window.location.hash.split("#")[1];if(e){var t=decodeURIComponent(e).split("\u20bf"),n=Object(c.a)(t,2),o=n[0],a=n[1];r({center:JSON.parse(o),zoom:a})}}),[]),Object(o.useEffect)((function(){window.location.hash=JSON.stringify(n.center)+"\u20bf"+n.zoom}),[n]),a.a.createElement("div",{className:"Mandelbrot"},j&&a.a.createElement("div",{style:{position:"absolute",left:"calc(50% - 200px)",top:50,zIndex:1e3}},a.a.createElement(R,{onClose:function(){return T(!1)}})),a.a.createElement("div",{style:{position:"absolute",right:17,bottom:17,zIndex:1e3}},a.a.createElement("div",{style:{display:"inline-block",verticalAlign:"bottom",marginRight:10}},a.a.createElement(S,{onClick:function(){N()("12psUNxtiCdE26y6DH7hje3bRHwUBeTyaz"),clearTimeout(E),v(setTimeout((function(){return d(!1)}),5e3)),d(!0),g("BTC address copied to clipboard. Thank you.")}})),a.a.createElement("div",{style:{display:"inline-block",verticalAlign:"bottom",marginRight:10}},a.a.createElement(x,{onClick:function(){return T(!j)}})),a.a.createElement("div",{style:{display:"inline-block",verticalAlign:"bottom"}},a.a.createElement(C,{onClick:function(){N()(decodeURIComponent(window.location.href)),clearTimeout(E),v(setTimeout((function(){return d(!1)}),5e3)),d(!0),g("Link copied to clipboard. Show people what you've found!")}}))),a.a.createElement(A.a,{anchorOrigin:{vertical:"top",horizontal:"center"},open:m,onClose:function(){return d(!1)},ContentProps:{"aria-describedby":"message-id"},width:100,message:a.a.createElement("span",null,f)}),a.a.createElement(l.a,{style:{height:"100%"},crs:u.a.CRS.Simple,bounds:B,viewport:n,onViewportChange:r,minZoom:0,tms:!0},a.a.createElement(k,{maxZoom:100,tileSize:256})))},L=n(62),W=n.n(L),M=n(89),H=n(90);function U(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);t&&(o=o.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,o)}return n}function J(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?U(n,!0).forEach((function(t){Object(M.a)(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):U(n).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}var Z=new Worker("/workers.js"),_=new Worker("/workers.js"),G={coords:{x:-1,y:0,z:1}},F=function(){return Object(o.useEffect)((function(){setTimeout((function(){Object(H.a)(W.a.mark((function e(){return W.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return null,e.next=3,new Promise((function(e,t){var n=0;Z.onmessage=function(t){1===++n&&console.timeEnd("rust"),e(t.data.imageData)},console.time("rust"),Array.from({length:1}).forEach((function(){return Z.postMessage(G)}))})).then((function(e){e}));case 3:return e.next=5,new Promise((function(e,t){var n=0;_.onmessage=function(t){1===++n&&console.timeEnd("js"),e(t.data.imageData)},console.time("js"),Array.from({length:100}).forEach((function(){return _.postMessage(J({},G,{computeOption:"js"}))}))})).then((function(e){}));case 5:case"end":return e.stop()}}),e)})))()}),1e3)}),[]),a.a.createElement("div",null,"benchmark")},V=Boolean("localhost"===window.location.hostname||"[::1]"===window.location.hostname||window.location.hostname.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/));function q(e){navigator.serviceWorker.register(e).then((function(e){e.onupdatefound=function(){var t=e.installing;t.onstatechange=function(){"installed"===t.state&&(navigator.serviceWorker.controller?console.log("New content is available; please refresh."):console.log("Content is cached for offline use."))}}})).catch((function(e){console.error("Error during service worker registration:",e)}))}i.a.render(a.a.createElement((function(){return"/benchmark"===window.location.pathname?a.a.createElement("div",{className:"App"},a.a.createElement("div",null,"Do the benchmark here"),a.a.createElement(F,null)):a.a.createElement("div",{className:"App"},a.a.createElement(I,null))}),null),document.getElementById("root")),function(){if("serviceWorker"in navigator){if(new URL("/mandelbrot-explorer",window.location).origin!==window.location.origin)return;window.addEventListener("load",(function(){var e="".concat("/mandelbrot-explorer","/service-worker.js");V?(!function(e){fetch(e).then((function(t){404===t.status||-1===t.headers.get("content-type").indexOf("javascript")?navigator.serviceWorker.ready.then((function(e){e.unregister().then((function(){window.location.reload()}))})):q(e)})).catch((function(){console.log("No internet connection found. App is running in offline mode.")}))}(e),navigator.serviceWorker.ready.then((function(){console.log("This web app is being served cache-first by a service worker. To learn more, visit https://goo.gl/SC7cgQ")}))):q(e)}))}}()},94:function(e,t,n){e.exports=n(205)},99:function(e,t,n){}},[[94,1,2]]]);
//# sourceMappingURL=main.a0ae6fea.chunk.js.map