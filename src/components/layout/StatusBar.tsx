import { Component, Show } from "solid-js";
import {
  currentPath,
  showLeftPanel,
  setShowLeftPanel,
  showRightPanel,
  setShowRightPanel,
} from "../../stores/app";

const StatusBar: Component = () => {
  return (
    <footer class="flex h-7 items-center justify-between border-t border-gray-200 bg-gray-50 px-4 text-xs text-gray-500 dark:border-gray-800 dark:bg-gray-900 dark:text-gray-400">
      <div class="flex items-center gap-2">
        <button
          class={`rounded px-1 hover:text-gray-700 dark:hover:text-gray-200 ${showLeftPanel() ? "text-blue-500" : ""}`}
          onClick={() => setShowLeftPanel(!showLeftPanel())}
          title="Toggle file panel"
        >
          ⊟
        </button>
        <Show when={currentPath()} fallback={<span>MarkRight v0.1.0</span>}>
          <span class="truncate">{currentPath()}</span>
        </Show>
      </div>
      <button
        class={`rounded px-1 hover:text-gray-700 dark:hover:text-gray-200 ${showRightPanel() ? "text-blue-500" : ""}`}
        onClick={() => setShowRightPanel(!showRightPanel())}
        title="Toggle TOC panel"
      >
        ⊟
      </button>
    </footer>
  );
};

export default StatusBar;
