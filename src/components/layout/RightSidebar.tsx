import { Component } from "solid-js";
import TocPanel from "../toc/TocPanel";

const RightSidebar: Component = () => {
  return (
    <aside class="h-full overflow-y-auto border-l border-gray-200 p-4 dark:border-gray-800">
      <h2 class="mb-4 text-sm font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
        On this page
      </h2>
      <TocPanel />
    </aside>
  );
};

export default RightSidebar;
