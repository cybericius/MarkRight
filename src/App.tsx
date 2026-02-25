import { Component, onMount } from "solid-js";
import ThreePanel from "./components/layout/ThreePanel";
import { loadConfig, loadLicense, handleInitialFile } from "./stores/actions";

const App: Component = () => {
  onMount(async () => {
    await loadConfig();
    loadLicense();
    handleInitialFile();
  });

  return <ThreePanel />;
};

export default App;
