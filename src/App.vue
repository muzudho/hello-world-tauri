<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// ビューモデル（View Model）。
const greetMsgVM = ref("");
const nameVM = ref("");
const num1VM = ref("");
const num2VM = ref("");
const num3VM = ref("");
const errorVM = ref("");

// イベントハンドラー。
async function on_greet_button_clicked() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgVM.value = await invoke("greet", { name: nameVM.value });
}

async function on_add_button_clicked() {
  num3VM.value = await invoke("add_two_numbers", { num1: num1VM.value, num2: num2VM.value });
  errorVM.value = "エラー無し。"
  /*
  num3VM.value = await invoke("add_two_numbers", { num1: num1VM.value, num2: num2VM.value });
   */
}
</script>

<template>
  <main class="container">
    <div class="flex items-center justify-center min-h-screen bg-gray-900 text-white">
      <h1 class="text-4xl font-bold text-fluent-blue">Hello, World!</h1>
    </div>

    <h1>Welcome to Tauri + Vue</h1>

    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="on_greet_button_clicked">
      <input id="greet-input" v-model="nameVM" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsgVM }}</p>

    <h1>テスト</h1>
    <form @submit.prevent="on_add_button_clicked">
      <input id="add-two-numbers-param1" v-model.number="num1VM" type="number" placeholder="数字1（例：2）"/>
      ＋
      <input id="add-two-numbers-param2" v-model.number="num2VM" type="number" placeholder="数字2（例：3）"/>
      <button type="submit">足し算</button><br/>
      答え：
      <input id="add-two-numbers-ret1" v-model="num3VM">
      <p v-if="errorVM" class="text-red-500 text-sm mt-2">
        {{ errorVM }}
      </p>
    </form>
  </main>
</template>

<style scoped>
.text-fluent-blue{
  color: #0078D4; /* Windows 11のFluent Design風カラー */
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>