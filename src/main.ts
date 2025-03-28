import { createApp } from "vue";
import App from "./App.vue";
import "./static/tailwind.css";
import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";
import Startup from "./routes/Startup.vue";

const routes: RouteRecordRaw[] = [
    { path: "/", component: Startup }
]

const router = createRouter({
    history: createWebHashHistory(),
    routes: routes,
})

createApp(App).use(router).mount("#app");
