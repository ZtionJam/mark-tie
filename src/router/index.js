import {createRouter, createWebHashHistory} from 'vue-router'
import main from '@/pages/main/mainPage'

const routes = [
    {
        path: "/",
        redirect: '/main',
        meta: {title: '马克贴'}
    },
    {
        path: "/main",
        component: main,
        meta: {title: 'Bamboo'}
    },
]
const router = createRouter({
    model: 'hash',
    history: createWebHashHistory(),
    routes: routes
})
export default router