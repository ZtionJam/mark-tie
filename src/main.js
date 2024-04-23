import { createApp } from "vue";
import "./styles.css";
import router from './router'
import App from "./App.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import { VueMasonryPlugin } from 'vue-masonry';
import { invoke } from "@tauri-apps/api/tauri";

router.beforeEach((to, from, next) => {
    
    if (to.meta.title) {
        document.title = to.meta.title
    }
    if(from.meta.saveScrollPosition){
        const index=router.options.routes.findIndex(item=>item.path===from.path)
        const position = { top:document.getElementById(from.meta.scrollBoxId).scrollTop }
        router.options.routes[index].meta.savePosition=position.top
    }
    next()
})


let app = createApp(App)
    .use(ElementPlus)
    .use(router)
    .use(VueMasonryPlugin);

//图片代理服务器配置
let config = await invoke("get_config")
app.config.globalProperties.img_proxy = url => {
    return "http://127.0.0.1:" + config.port + "/img?url=" + encodeURIComponent(url);
};
app.config.globalProperties.img_proxy_list = function(urls) {
    return Array.from(urls).map(u => {
        return "http://127.0.0.1:" + config.port + "/img?url=" + encodeURIComponent(u);
    })

};


app.mount('#app')
