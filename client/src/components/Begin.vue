<template>
    <div>
        <p>Choose your e-commerce solution </p>
        <img class="shop" src="../assets/shopifyLogo.png" alt="shopify">
    </div>
</template>

<script>

export default {
  name: 'Begin',
 
}
</script>
<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
p {
  margin: 7% 5%;
  font-size: 2em;
}
.shop {
  margin: 1% 0 1% 17%; 
  opacity: .5;
  cursor: pointer;

}
.shop:hover {
	-webkit-transform: scale(1.3);
	transform: scale(1.3);
  opacity: 1;
}
</style>
