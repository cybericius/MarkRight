import { Component, Show } from "solid-js";
import {
  currentPath,
  showLeftPanel,
  showRightPanel,
  setShowSettings,
  isLicensed,
  licenseStatus,
} from "../../stores/app";
import { updateConfig } from "../../stores/actions";

const StatusBar: Component = () => {
  return (
    <footer class="flex h-7 items-center justify-between border-t border-gray-200 bg-gray-50 px-4 text-xs text-gray-500 dark:border-gray-800 dark:bg-gray-900 dark:text-gray-400">
      <div class="flex items-center gap-2">
        <button
          class={`rounded px-1 hover:text-gray-700 dark:hover:text-gray-200 ${showLeftPanel() ? "text-blue-500" : ""}`}
          onClick={() => updateConfig({ show_left_panel: !showLeftPanel() })}
          title="Toggle file panel"
        >
          ⊟
        </button>
        <Show when={currentPath()} fallback={<span>MarkRight v0.1.0</span>}>
          <span class="truncate">{currentPath()}</span>
        </Show>
      </div>
      <div class="flex items-center gap-2">
        <Show when={isLicensed()}>
          <span class="text-green-600 dark:text-green-400">
            Registered to {licenseStatus().email}
          </span>
        </Show>
        <button
          class="rounded px-1 hover:text-gray-700 dark:hover:text-gray-200"
          onClick={() => setShowSettings(true)}
          title="Settings (Ctrl+,)"
        >
          ⚙
        </button>
        <button
          class={`rounded px-1 hover:text-gray-700 dark:hover:text-gray-200 ${showRightPanel() ? "text-blue-500" : ""}`}
          onClick={() => updateConfig({ show_right_panel: !showRightPanel() })}
          title="Toggle TOC panel"
        >
          ⊟
        </button>
      </div>
    </footer>
  );
};

export default StatusBar;
