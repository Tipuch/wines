import Vue from 'vue';
import vuetify from './plugins/vuetify.js';
import App from './App.vue';

new Vue({
    vuetify: vuetify,
    el: '#app',
    render: h => h(App)
});
