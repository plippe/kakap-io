<template>
    <div id="home-page">
        <h1>Une solution pour optimiser le shopping en ligne</h1>
        <br>
        <div class="bloctext">
            <p>Le kakapo, est un oiseau au plumage vert et jaune. Il a pour qualité d'être plein de ressources mais surtout il
            sait se faire entendre de très loin.
            <br>  
            Tout comme ce spécimen tout en plumes, Kakap-io saura vous faire voire et renforcer la notoriété de votre site.
            </p>
            <br>
            <h2>Nous vous aiderons à diffuser vos produits et toucher un maximum d'internautes qui effectuent 
            chaque jour des recherches liées à un achat sur Google
            </h2>
        </div>
        <div class="bloc">
            <img src="../assets/ecommerce-marketing.png" alt="retail">
        <button class="button" @click="startApp"> Let's go!</button></div>
        
    </div>
</template>

<script>
import Router from "@/router";
import Begin from "./Begin"

export default {
  name: 'Home',
  component: {Begin},
  data() {
    return { }
  },
  methods: {
    startApp(){
      Router.push({ name: "Begin" });
    }
  },
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

#home-page {
  margin-top: 0 auto;
  height: 100%;
  width: 100%;
}
h1 {
    text-align: center;
    margin-top: 6%;
    font-size: 40px;
    line-height: 1.25;
    font-family: Arial, Helvetica, sans-serif;
}
h2 {
    text-indent: 50px;
    max-width: 80%;
    line-height: 1.25;
    
}
.bloctext{
    margin-left: 4%;
    max-width: 60%;
    margin-top: 20px;
    float: left;
}
.bloc {
    display: block;
    margin-left: 50%;
}
.button {
    display: block;
    background-color: #4CAF50; /* Green */
    border: none;
    color: white;
    padding: 24px 32px;
    text-align: center;
    text-decoration: none;
    font-size: 16px;
    margin: 4px 2px;
    cursor: pointer;
}
p {
    text-indent: 50px;
    text-align: justify;
    font-size: 1.4em;
    line-height: 1.5;
    font-family: Arial, Helvetica, sans-serif;
}

</style>