import { Component } from "solid-js";

const StatusBar: Component = () => {
  return (
    <footer class="flex h-7 items-center border-t border-gray-200 bg-gray-50 px-4 text-xs text-gray-500 dark:border-gray-800 dark:bg-gray-900 dark:text-gray-400">
      <span>MarkRight v0.1.0</span>
    </footer>
  );
};

export default StatusBar;
