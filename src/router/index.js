import {createRouter, createWebHashHistory} from 'vue-router'
import main from '@/pages/main/mainPage'
import login from '@/pages/login/loginPage'

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
            title: '马克贴'
        }
    },
    {
        path: "/login",
        component: login,
        name: "loginPage",
        meta: {
            title: '马克贴'
        }
    }
]
const router = createRouter({
    model: 'hash',
    history: createWebHashHistory(),
    routes: routes
})
export default router