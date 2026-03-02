import { Component, Show, createMemo } from "solid-js";
import type { MdNode } from "../../lib/types";
import { isDark } from "../../stores/app";

const CodeBlock: Component<{ node: MdNode }> = (props) => {
  const html = createMemo(() =>
    isDark() ? props.node.highlighted_html : (props.node.highlighted_html_light ?? props.node.highlighted_html)
  );

  return (
    <Show
      when={html()}
      fallback={
        <pre class="overflow-x-auto rounded-lg bg-gray-100 p-4 text-sm text-gray-800 dark:bg-gray-900 dark:text-gray-100">
          <code>{props.node.literal ?? ""}</code>
        </pre>
      }
    >
      <div
        class="code-block overflow-x-auto rounded-lg text-sm [&>pre]:p-4"
        innerHTML={html()!}
      />
    </Show>
  );
};

export default CodeBlock;
