import { Component, For, createSignal, Show } from "solid-js";
import type { TreeNode } from "../../lib/types";
import { openDocument } from "../../stores/actions";
import { currentPath } from "../../stores/app";

const TreeItem: Component<{ node: TreeNode }> = (props) => {
  const [expanded, setExpanded] = createSignal(false);

  const handleClick = () => {
    if (props.node.is_dir) {
      setExpanded(!expanded());
    } else {
      openDocument(props.node.path);
    }
  };

  const isActive = () => currentPath() === props.node.path;

  return (
    <li>
      <button
        class={`flex w-full items-center gap-1 rounded px-2 py-0.5 text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-800 ${
          isActive()
            ? "bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300"
            : "text-gray-700 dark:text-gray-300"
        }`}
        onClick={handleClick}
      >
        <span class="shrink-0 text-xs text-gray-400">
          {props.node.is_dir ? (expanded() ? "▾" : "▸") : "·"}
        </span>
        <span class="truncate">{props.node.name}</span>
      </button>
      <Show when={props.node.is_dir && expanded()}>
        <ul class="ml-3 border-l border-gray-200 pl-1 dark:border-gray-700">
          <For each={props.node.children}>
            {(child) => <TreeItem node={child} />}
          </For>
        </ul>
      </Show>
    </li>
  );
};

export default TreeItem;
