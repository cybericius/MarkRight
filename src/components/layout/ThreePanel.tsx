import { Component } from "solid-js";
import LeftSidebar from "./LeftSidebar";
import ContentArea from "./ContentArea";
import RightSidebar from "./RightSidebar";
import StatusBar from "./StatusBar";

const ThreePanel: Component = () => {
  return (
    <div class="flex h-screen flex-col bg-white text-gray-900 dark:bg-gray-950 dark:text-gray-100">
      <div class="flex min-h-0 flex-1">
        <LeftSidebar />
        <ContentArea />
        <RightSidebar />
      </div>
      <StatusBar />
    </div>
  );
};

export default ThreePanel;
