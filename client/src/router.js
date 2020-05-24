import Vue from 'vue'
import Router from 'vue-router'

import Home from './components/Home.vue'
import Begin from './components/Begin.vue'

Vue.use(Router)

const vueRouter = new Router({
    mode: 'history',
    routes: [
        {
            path: '/',
            alias: '/Home',
            name: 'Home',
            component: Home,
            meta: {
                title: 'home'
            }

        },
        {
            path: '/Begin',
            name: 'Begin',
            component: Begin
        }
    ]
})

export default vueRouter