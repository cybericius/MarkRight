import { Component, Show, createEffect, createSignal, on } from "solid-js";
import { isDark } from "../../stores/app";

// Lazy-load mermaid: it is large and only needed for documents with diagrams.
let mermaidPromise: Promise<typeof import("mermaid").default> | undefined;
let renderSeq = 0;

function loadMermaid() {
  mermaidPromise ??= import("mermaid").then((m) => m.default);
  return mermaidPromise;
}

const MermaidBlock: Component<{ source: string }> = (props) => {
  const [svg, setSvg] = createSignal<string>();
  const [error, setError] = createSignal<string>();
  let latest = "";

  createEffect(
    on([() => props.source, isDark], async ([source, dark]) => {
      const id = `mermaid-${++renderSeq}`;
      latest = id;
      try {
        const mermaid = await loadMermaid();
        mermaid.initialize({
          startOnLoad: false,
          securityLevel: "strict",
          theme: dark ? "dark" : "default",
        });
        const { svg: out } = await mermaid.render(id, source);
        if (latest !== id) return;
        setSvg(out);
        setError(undefined);
      } catch (e) {
        // mermaid leaves its error SVG in the body on failure
        document.getElementById(`d${id}`)?.remove();
        if (latest !== id) return;
        setError(e instanceof Error ? e.message : String(e));
        setSvg(undefined);
      }
    })
  );

  return (
    <Show
      when={!error()}
      fallback={
        <div class="rounded-lg border border-red-300 p-4 text-[0.85em] dark:border-red-800">
          <div class="mb-2 font-semibold text-red-600 dark:text-red-400">Mermaid error</div>
          <pre class="overflow-x-auto whitespace-pre-wrap text-red-700 dark:text-red-300">{error()}</pre>
          <pre class="mt-2 overflow-x-auto text-gray-700 dark:text-gray-300">
            <code>{props.source}</code>
          </pre>
        </div>
      }
    >
      <div class="mermaid-block my-4 flex justify-center overflow-x-auto" innerHTML={svg() ?? ""} />
    </Show>
  );
};

export default MermaidBlock;
