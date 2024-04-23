import {createApp} from "vue";
import "./styles.css";
import router from './router'
import App from "./App.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import { VueMasonryPlugin } from 'vue-masonry';

router.beforeEach((to, from, next) => {
    if (to.meta.title) {
        document.title = to.meta.title
    }
    next()
})

createApp(App)
    .use(ElementPlus)
    .use(router)
    .use(VueMasonryPlugin)
    .mount('#app')
