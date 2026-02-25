import { Component, For, Show } from "solid-js";
import { tree } from "../../stores/app";
import TreeItem from "./TreeItem";

const FileTree: Component = () => {
  return (
    <Show
      when={tree().length > 0}
      fallback={
        <p class="text-sm text-gray-400 dark:text-gray-500">
          Open a folder to browse markdown files.
        </p>
      }
    >
      <ul class="space-y-0.5">
        <For each={tree()}>{(node) => <TreeItem node={node} />}</For>
      </ul>
    </Show>
  );
};

export default FileTree;
