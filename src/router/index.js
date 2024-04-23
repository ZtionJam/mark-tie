import { createRouter, createWebHashHistory } from 'vue-router'
import main from '@/pages/main/mainPage'
import feed from '@/pages/feed/feedPage'

const routes = [
    {
        path: "/",
        redirect: '/main',
        meta: { title: '马克贴' },
        meta: { keepAlive: true }
    },
    {
        path: "/main",
        component: main,
        meta: { title: '马克贴' },
        meta: {
            keepAlive: true,
            saveScrollPosition: true,
            scrollBoxId: "box",
            savePosition:0
        }

    },
    {
        path: "/feed",
        component: feed,
        meta: { title: '马克贴' }
    },
]
const router = createRouter({
    model: 'hash',
    history: createWebHashHistory(),
    routes: routes
})
export default router