import { Component, onMount } from "solid-js";
import ThreePanel from "./components/layout/ThreePanel";
import { loadConfig, loadLicense } from "./stores/actions";

const App: Component = () => {
  onMount(() => {
    loadConfig();
    loadLicense();
  });

  return <ThreePanel />;
};

export default App;
