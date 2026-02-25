import { Component } from "solid-js";

const LeftSidebar: Component = () => {
  return (
    <aside class="w-64 shrink-0 overflow-y-auto border-r border-gray-200 p-4 dark:border-gray-800">
      <h2 class="mb-4 text-sm font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
        Navigation
      </h2>
      <p class="text-sm text-gray-400 dark:text-gray-500">
        Open a folder to browse markdown files.
      </p>
    </aside>
  );
};

export default LeftSidebar;
