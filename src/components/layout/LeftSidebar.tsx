import { Component, Show, onMount, onCleanup } from "solid-js";
import { open } from "@tauri-apps/plugin-dialog";
import { openFolder, toggleSearchMode, promptUpgrade } from "../../stores/actions";
import { searchMode, setShowSettings, showSettings, isLicensed } from "../../stores/app";
import FileTree from "../tree/FileTree";
import SearchPanel from "../search/SearchPanel";
import ProBadge from "../license/ProBadge";

const LeftSidebar: Component = () => {
  const handleOpenFolder = async () => {
    const selected = await open({ directory: true, multiple: false });
    if (selected) {
      await openFolder(selected as string);
    }
  };

  const handleSearchToggle = () => {
    if (isLicensed()) {
      toggleSearchMode();
    } else {
      promptUpgrade("Cross-File Search");
    }
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.ctrlKey && e.shiftKey && e.key === "F") {
      e.preventDefault();
      handleSearchToggle();
    }
    if (e.ctrlKey && e.key === ",") {
      e.preventDefault();
      setShowSettings(!showSettings());
    }
  };

  onMount(() => window.addEventListener("keydown", handleKeyDown));
  onCleanup(() => window.removeEventListener("keydown", handleKeyDown));

  return (
    <aside class="flex h-full flex-col overflow-hidden border-r border-gray-200 dark:border-gray-800">
      <div class="flex items-center justify-between border-b border-gray-200 px-4 py-2 dark:border-gray-800">
        <h2 class="text-sm font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
          <Show when={searchMode()} fallback="Files">
            Search
          </Show>
        </h2>
        <div class="flex items-center gap-1">
          <button
            class="flex items-center gap-1 rounded px-1.5 py-1 text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800"
            onClick={handleSearchToggle}
            title="Cross-File Search (Ctrl+Shift+F)"
          >
            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <Show
                when={searchMode()}
                fallback={
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                  />
                }
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M6 18L18 6M6 6l12 12"
                />
              </Show>
            </svg>
            <Show when={!isLicensed()}>
              <ProBadge />
            </Show>
          </button>
          <button
            class="rounded px-2 py-1 text-xs font-medium text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800"
            onClick={handleOpenFolder}
          >
            Open Folder
          </button>
        </div>
      </div>
      <div class="flex-1 overflow-y-auto p-2">
        <Show when={searchMode()} fallback={<FileTree />}>
          <SearchPanel />
        </Show>
      </div>
    </aside>
  );
};

export default LeftSidebar;
