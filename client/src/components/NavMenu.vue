<template>
  <div class="header">
      <img id="logo" @click="goToHome" src="../assets/logoKakapio.png" alt="kakap-io">
  </div>
</template>

<script>
import Router from "@/router";

export default {
  name: 'NavMenu',
  methods: {
    goToHome() {
      Router.push({ name: "Home" });
    }
    },
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

.header {
  margin-top: 0 auto;
  height: 100%;
  width: 100%;
  -webkit-box-shadow: 0px 7px 54px -13px rgba(192,192,192,1);
  -moz-box-shadow: 0px 7px 54px -13px rgba(192,192,192,1);
  box-shadow: 0px 7px 54px -13px rgba(192,192,192,1);
}
#logo {
    margin-left: 50px;
    margin-top: 20px;
    cursor: pointer;

}
</style>
