import { Component, Show } from "solid-js";
import type { MdNode } from "../../lib/types";

const CodeBlock: Component<{ node: MdNode }> = (props) => {
  return (
    <Show
      when={props.node.highlighted_html}
      fallback={
        <pre class="overflow-x-auto rounded-lg bg-gray-100 p-4 text-sm text-gray-800 dark:bg-gray-900 dark:text-gray-100">
          <code>{props.node.literal ?? ""}</code>
        </pre>
      }
    >
      <div
        class="code-block overflow-x-auto rounded-lg text-sm [&>pre]:p-4"
        innerHTML={props.node.highlighted_html!}
      />
    </Show>
  );
};

export default CodeBlock;
