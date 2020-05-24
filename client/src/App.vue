<template>
  <div id="app">
    <NavMenu />
      <div id="main">
        <div class="router">
          <router-view />
        </div>
      </div>
  </div>
</template>

<script>
import NavMenu from './components/NavMenu'

export default {
  name: 'App',
  components: {
    NavMenu
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  margin: 0 auto;
}
#main {
  margin-top: 70px;
  
}

</style>
