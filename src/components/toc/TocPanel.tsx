import { Component, For, Show } from "solid-js";
import { toc, activeTocId, setActiveTocId } from "../../stores/app";

const TocPanel: Component = () => {
  const handleClick = (id: string) => {
    setActiveTocId(id);
    const el = document.getElementById(id);
    el?.scrollIntoView({ behavior: "smooth" });
  };

  return (
    <Show
      when={toc().length > 0}
      fallback={
        <p class="text-sm text-gray-400 dark:text-gray-500">
          No headings found.
        </p>
      }
    >
      <nav>
        <ul class="space-y-0.5">
          <For each={toc()}>
            {(entry) => (
              <li>
                <button
                  class={`block w-full rounded px-2 py-0.5 text-left text-sm transition-colors hover:bg-gray-100 dark:hover:bg-gray-800 ${
                    activeTocId() === entry.id
                      ? "bg-blue-50 font-medium text-blue-700 dark:bg-blue-900/30 dark:text-blue-300"
                      : "text-gray-600 dark:text-gray-400"
                  }`}
                  style={{ "padding-left": `${(entry.level - 1) * 12 + 8}px` }}
                  onClick={() => handleClick(entry.id)}
                >
                  {entry.text}
                </button>
              </li>
            )}
          </For>
        </ul>
      </nav>
    </Show>
  );
};

export default TocPanel;
