import {createRouter, createWebHashHistory} from 'vue-router'
import main from '@/pages/main/mainPage'
import feed from '@/pages/feed/feedPage'

const routes = [
    {
        path: "/",
        redirect: '/main'
    },
    {
        path: "/main",
        component: main,
        name: "mainPage",
        meta: {
            title: '马克贴',
            keepAlive: true,
            saveScrollPosition: true,
            scrollBoxId: "box",
            savePosition: 0
        }

    },
    {
        path: "/feed",
        component: feed,
        name: "feedPage",
        meta: {
            title: '马克贴',
            keepAlive: false
        }
    },
]
const router = createRouter({
    model: 'hash',
    history: createWebHashHistory(),
    routes: routes
})
export default router