<template>
  <nav>
    <router-link to="/">Главная</router-link> |
    <router-link to="/profile">Профиль</router-link> |
    <router-link to="/schedule">Расписание</router-link> |
    <router-link to="/instructors">Инструкторы</router-link> |
    <router-link to="/more">Еще</router-link> |
  </nav>
  <router-view/>
</template>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>
