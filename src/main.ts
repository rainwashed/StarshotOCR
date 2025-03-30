import { createApp } from "vue";
import App from "./App.vue";
import "./static/tailwind.css";
import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import Startup from "./routes/Startup.vue";
import Home from "./routes/Home.vue";

const routes: RouteRecordRaw[] = [
    { path: "/", component: Home },
    { path: "/startup", component: Startup }
]

const router = createRouter({
    history:  createWebHistory(),
    routes: routes,
})

createApp(App).use(router).mount("#app");
