<template>
  <header id="header"></header>
  <router-view />
  <footer class="navbar" v-if="isAuthenticated">
    <div class="nav-btn">
      <router-link class="nav-link" to="/">
        <svg width="28" height="28" xmlns="http://www.w3.org/2000/svg" fill="none">
          <path stroke="null" id="svg_1" fill-opacity="0.5" fill="currentColor"
            d="m27.70067,12.29453l-13.01893,-12.02919c-0.3792,-0.35379 -0.98413,-0.35379 -1.36344,0l-13.01841,12.02872c-0.19165,0.17702 -0.29988,0.42097 -0.29988,0.67589c0,0.51883 0.43951,0.93937 0.9816,0.93937l1.73005,0l0,13.15128c0,0.51884 0.43945,0.9394 0.9816,0.9394l6.87117,0c0.54215,0 0.9816,-0.42056 0.9816,-0.9394l0,-7.51506l4.90797,0l0,7.51506c0,0.51884 0.43947,0.9394 0.9816,0.9394l6.8712,0c0.54213,0 0.9816,-0.42056 0.9816,-0.9394l0,-13.15128l1.73,0c0.26613,0 0.5208,-0.10339 0.70573,-0.28651c0.3768,-0.37306 0.36627,-0.96774 -0.02347,-1.32828zm-2.41226,0.67542c-0.54214,0 -0.9816,0.42055 -0.9816,0.93937l0,13.15128l-6.8712,0l0,-7.51506c0,-0.51884 -0.43947,-0.9394 -0.9816,-0.9394l-4.90797,0c-0.54215,0 -0.9816,0.42056 -0.9816,0.9394l0,7.51506l-6.87117,0l0,-13.15128c0,-0.51883 -0.43944,-0.93937 -0.9816,-0.93937l-1.73005,0l13.0184,-12.02873l13.02333,12.02873l-1.73493,0z" />
        </svg>
        Главная
      </router-link>
    </div>
    <div class="nav-btn">
      <router-link class="nav-link" to="/profile">
        <svg width="28" height="28" xmlns="http://www.w3.org/2000/svg" fill="none">
          <defs>
            <clipPath id="clip0_136_166">
              <rect width="28" height="28" fill="white" id="svg_1" />
            </clipPath>
          </defs>
          <g>
            <title>Layer 1</title>
            <g clip-path="url(#clip0_136_166)" id="svg_2">
              <path
                d="m15.06247,13.36491c2.27151,0 4.11291,-2.16639 4.11291,-4.83871c0,-2.67232 -1.8414,-4.83871 -4.11291,-4.83871c-2.27147,0 -4.11291,2.16639 -4.11291,4.83871c0.00255,2.67106 1.84251,4.83572 4.11291,4.83871zm0,-9.03226c1.96861,0 3.56451,1.87752 3.56451,4.19355c0,2.31603 -1.5959,4.19355 -3.56451,4.19355c-1.96863,0 -3.56452,-1.87752 -3.56452,-4.19355c0.00228,-2.31493 1.59683,-4.19087 3.56452,-4.19355z"
                fill-opacity="0.5" fill="currentColor" id="svg_3" />
              <path
                d="m15.61087,14.01009l-1.09678,0c-4.38937,0.006 -7.94652,4.1909 -7.95161,9.3548c0,0.1782 0.12277,0.3226 0.27419,0.3226l16.45161,0c0.1514,0 0.2742,-0.1444 0.2742,-0.3226c-0.0051,-5.1639 -3.5622,-9.3488 -7.95161,-9.3548zm-8.49478,9.0322c0.15182,-4.6791 3.41778,-8.3819 7.398,-8.3871l1.09678,0c3.98021,0.0052 7.24621,3.708 7.39801,8.3871l-15.89279,0z"
                fill-opacity="0.5" fill="currentColor" id="svg_4" />
              <path
                d="m19.17538,22.07459c0,0.1781 0.1228,0.3226 0.2742,0.3226l1.6452,0c0.1514,0 0.2742,-0.1445 0.2742,-0.3226c0,-0.1781 -0.1228,-0.3226 -0.2742,-0.3226l-1.6452,0c-0.1514,0 -0.2742,0.1445 -0.2742,0.3226z"
                fill-opacity="0.5" fill="currentColor" id="svg_5" />
              <path
                d="m21.91728,22.39719l0.2742,0c0.1514,0 0.2742,-0.1445 0.2742,-0.3226c0,-0.1781 -0.1228,-0.3226 -0.2742,-0.3226l-0.2742,0c-0.1514,0 -0.2742,0.1445 -0.2742,0.3226c0,0.1781 0.1228,0.3226 0.2742,0.3226z"
                fill-opacity="0.5" fill="currentColor" id="svg_6" />
            </g>
          </g>
        </svg>
        Профиль</router-link>
    </div>
    <div class="nav-btn">
      <router-link class="nav-link" to="/schedule">
        <svg width="28" height="28" xmlns="http://www.w3.org/2000/svg" fill="none">
          <g>
            <title>Layer 1</title>
            <path id="svg_1" fill-opacity="0.5" fill="currentColor"
              d="m22.09379,4.69263l-1.8033,0l0,-1.15068c0,-0.19454 -0.1249,-0.35225 -0.2789,-0.35225c-0.154,0 -0.2788,0.15771 -0.2788,0.35225l0,1.15068l-2.4169,0l0,-1.15068c0,-0.19454 -0.1248,-0.35225 -0.2788,-0.35225c-0.154,0 -0.2789,0.15771 -0.2789,0.35225l0,1.15068l-2.41684,0l0,-1.15068c0,-0.19454 -0.12485,-0.35225 -0.27886,-0.35225c-0.15401,0 -0.27886,0.15771 -0.27886,0.35225l0,1.15068l-2.41683,0l0,-1.15068c0,-0.19454 -0.12486,-0.35225 -0.27887,-0.35225c-0.15401,0 -0.27886,0.15771 -0.27886,0.35225l0,1.15068l-2.41683,0l0,-1.15068c0,-0.19454 -0.12486,-0.35225 -0.27887,-0.35225c-0.15401,0 -0.27886,0.15771 -0.27886,0.35225l0,1.15068l-1.80333,0c-0.80982,0 -1.46869,0.83225 -1.46869,1.85519l0,16.53228c0,1.0229 0.65887,1.8552 1.46869,1.8552l16.06261,0c0.8098,0 1.4687,-0.8323 1.4687,-1.8552l0,-16.53228c0,-1.02294 -0.6589,-1.85519 -1.4687,-1.85519zm0.911,18.38747c0,0.6345 -0.4087,1.1507 -0.911,1.1507l-16.06261,0c-0.50229,0 -0.91096,-0.5162 -0.91096,-1.1507l0,-16.53228c0,-0.63447 0.40867,-1.15069 0.91096,-1.15069l1.80333,0l0,1.15069c0,0.19454 0.12485,0.35225 0.27886,0.35225c0.15401,0 0.27887,-0.15771 0.27887,-0.35225l0,-1.15069l2.41683,0l0,1.15069c0,0.19454 0.12485,0.35225 0.27886,0.35225c0.15401,0 0.27887,-0.15771 0.27887,-0.35225l0,-1.15069l2.41683,0l0,1.15069c0,0.19454 0.12485,0.35225 0.27886,0.35225c0.15401,0 0.27886,-0.15771 0.27886,-0.35225l0,-1.15069l2.41684,0l0,1.15069c0,0.19454 0.1249,0.35225 0.2789,0.35225c0.154,0 0.2788,-0.15771 0.2788,-0.35225l0,-1.15069l2.4169,0l0,1.15069c0,0.19454 0.1248,0.35225 0.2788,0.35225c0.154,0 0.2789,-0.15771 0.2789,-0.35225l0,-1.15069l1.8033,0c0.5023,0 0.911,0.51622 0.911,1.15069l0,16.53228z" />
            <path id="svg_2" fill-opacity="0.5" fill="currentColor"
              d="m22.09379,8.44996l-16.06262,0c-0.154,0 -0.27886,0.15771 -0.27886,0.35225c0,0.19453 0.12486,0.35225 0.27886,0.35225l16.06262,0c0.154,0 0.2789,-0.15772 0.2789,-0.35225c0,-0.19454 -0.1249,-0.35225 -0.2789,-0.35225z" />
            <path id="svg_3" fill-opacity="0.5" fill="currentColor"
              d="m18.92209,12.6862l-5.82595,7.3591c-0.12322,0.1556 -0.32371,0.1556 -0.44697,0l-2.85137,-3.6018c-0.10891,-0.1375 -0.28548,-0.1375 -0.39435,0c-0.10891,0.1376 -0.10891,0.3606 0,0.4982l2.85137,3.6017c0.16502,0.2084 0.38443,0.3233 0.61786,0.3233c0.23342,0 0.4528,-0.1148 0.61785,-0.3233l5.82596,-7.3591c0.1089,-0.1375 0.1089,-0.3606 0,-0.4981c-0.1089,-0.1375 -0.2855,-0.1375 -0.3944,0z" />
          </g>
        </svg>
        Расписание
      </router-link>
    </div>
    <div class="nav-btn">
      <router-link class="nav-link" to="/more">
        <svg width="28" height="28" xmlns="http://www.w3.org/2000/svg" fill="none">
          <defs>
            <clipPath id="clip0_136_163">
              <rect id="svg_1" fill="white" height="28" width="28" />
            </clipPath>
          </defs>
          <g>
            <title>Layer 1</title>
            <g stroke="null" id="svg_2" clip-path="url(#clip0_136_163)">
              <path stroke="null" id="svg_3" fill-opacity="0.5" fill="currentColor"
                d="m23.2274,15.97658c0.1634,0 0.296,-0.1589 0.296,-0.3559l0,-5.7021c0,-0.43243 -0.2925,-0.7845 -0.6524,-0.7845l-0.8451,0l0,-2.05828c0,-0.43224 -0.2925,-0.78449 -0.6524,-0.78449l-0.8451,0l0,-2.05773c0,-0.43279 -0.2929,-0.78449 -0.6524,-0.78449l-7.32836,0c-0.16327,0 -0.29602,0.15907 -0.29602,0.35592c0,0.19649 0.13275,0.35592 0.29602,0.35592l7.32836,0c0.0331,0 0.0603,0.03229 0.0603,0.07265l0,2.05773l-14.2629,0c-0.35995,0 -0.65246,0.35225 -0.65246,0.78449l0,10.12698l-0.84504,0c-0.03357,0 -0.06042,-0.0326 -0.06042,-0.0724l0,-12.8968c0,-0.04036 0.02685,-0.07265 0.06042,-0.07265l7.18766,0c0.16388,0 0.29602,-0.15943 0.29602,-0.35592c0,-0.19685 -0.13214,-0.35592 -0.29602,-0.35592l-7.18766,0c-0.35995,0 -0.65246,0.3517 -0.65246,0.78449l0,12.8968c0,0.4322 0.29251,0.7845 0.65246,0.7845l0.84504,0l0,2.0581c0,0.4324 0.29251,0.7845 0.65246,0.7845l0.84503,0l0,2.0579c0,0.4328 0.29297,0.7845 0.65247,0.7845l15.7001,0c0.3599,0 0.6524,-0.3517 0.6524,-0.7845l0,-5.7711c0,-0.1965 -0.1326,-0.3559 -0.296,-0.3559c-0.1634,0 -0.296,0.1594 -0.296,0.3559l0,5.7711c0,0.0401 -0.0269,0.0724 -0.0604,0.0724l-15.7001,0c-0.03311,0 -0.06027,-0.0323 -0.06027,-0.0724c0,-0.923 0,-7.8798 0,-8.9246c0,-0.1965 -0.13276,-0.3559 -0.29602,-0.3559c-0.16343,0 -0.29618,0.1594 -0.29618,0.3559c0,1.2248 0,4.9115 0,6.1548l-0.84503,0c-0.03311,0 -0.06042,-0.0326 -0.06042,-0.0726c0,-0.6238 0,-12.39247 0,-12.89718c0,-0.03981 0.02731,-0.07265 0.06042,-0.07265c1.63818,0 14.8975,0 15.7001,0c0.0331,0 0.0604,0.03284 0.0604,0.07265l0,2.05828c-0.7851,0 -13.69766,0 -14.263,0c-0.3595,0 -0.65247,0.35207 -0.65247,0.7845l0,2.5485c0,0.1968 0.13275,0.3559 0.29618,0.3559c0.16326,0 0.29602,-0.1591 0.29602,-0.3559l0,-2.5485c0,-0.03982 0.02716,-0.07266 0.06027,-0.07266c0.57648,0 14.866,0 15.7001,0c0.0335,0 0.0604,0.03284 0.0604,0.07266l0,5.7021c0,0.197 0.1326,0.3559 0.296,0.3559z" />
            </g>
          </g>
        </svg>
        Еще
      </router-link>
    </div>
  </footer>
  <Popup />
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, watch } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { getAuthService } from '@/services/auth';
import Popup from '@/components/common/PopupView.vue';


export default defineComponent({
  components: {
    Popup
  },
  name: 'App',
  setup() {
    const router = useRouter();
    const route = useRoute();
    const auth = getAuthService();


    const isAuthenticated = ref(false);

    watch(
      () => auth.isAuthenticated(),
      (newValue) => {
        isAuthenticated.value = newValue;

        if (!newValue && route.path !== '/auth') {
          router.push('/auth');
        }
      }
    );

    onMounted(async () => {
      auth.checkAuthStatus();
      console.log('Auth status checked');
      isAuthenticated.value = auth.isAuthenticated();
      if (!isAuthenticated.value && route.path !== '/auth') {
        if (auth.getAuthType() === 'telegram') {
          await router.push('/tg-auth');
        } else {
          await router.push('/auth');
        }
      }
    });

    watch(
      () => route.path,
      async (newPath) => {
        isAuthenticated.value = auth.isAuthenticated();
        if (!isAuthenticated.value && newPath !== '/auth') {
          if (auth.getAuthType() === 'telegram') {
            await router.push('/tg-auth');
          } else {
            await router.push('/auth');
          }
        }
      }
    );

    return {
      isAuthenticated
    };
  }
});
</script>

<style lang="scss">
body,
html {
  margin: 0;
  padding: 0;
  height: 100%;
  --bg-color: var(--tg-theme-bg-color);
  font: 12px/18px "Lucida Grande", "Lucida Sans Unicode", Arial, Helvetica, Verdana, sans-serif;
  background-color: var(--bg-color);
  color: var(--tg-theme-text-color);
  color-scheme: var(--tg-color-scheme);
  overflow: hidden;
  -webkit-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

header,
footer {
  text-align: center;
  padding: 10px 0;
  position: fixed;
  width: 100%;
  z-index: 1000;
}

header {
  top: 0;
}

footer {
  bottom: 0;
}

.navbar {
  display: flex;
  justify-content: center;
  align-items: center;
  background: var(--tg-theme-bg-color);
  border-top: 1px solid var(--tg-theme-hint-color);
  padding-bottom: 10px;
  box-sizing: border-box;
  padding-top: 5px;
  padding-bottom: 10px;
}

.nav-btn {
  flex: 1;
  padding: 10px;
  border: none;
  background: var(--tg-theme-bg-color);
  color: var(--tg-theme-text-color);
  font-size: 12px;
  cursor: pointer;
}

.content {
  position: absolute;
  top: 50px;
  bottom: 50px;
  left: 0;
  right: 0;
  overflow-x: hidden;
  overflow-y: scroll;
  padding: 10px;
  box-sizing: border-box;
  background-color: #f8f8f8;
}

.pull-to-refresh {
  display: none;
  text-align: center;
  padding: 10px;
  background-color: #ddd;
  position: absolute;
  width: 100%;
  top: 0;
}

.placeholder {
  height: 120vh;
}

a {
  color: #2c3e50;

  &.router-link-exact-active {
    color: #e24c4c;
  }
}

.nav-link {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-decoration: none;
}
</style>
