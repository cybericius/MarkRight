import { Component, Show, createEffect, createSignal, on } from "solid-js";
import type { MdNode } from "../../lib/types";

// Lazy-load KaTeX and its stylesheet (fonts are bundled by Vite): only
// documents containing math pay for them.
let katexPromise: Promise<typeof import("katex").default> | undefined;

function loadKatex() {
  katexPromise ??= Promise.all([
    import("katex"),
    // CSS side-effect import: no type declarations ship for it in this project.
    // @ts-ignore TS2307 (tolerant if vite/client types are added later)
    import("katex/dist/katex.min.css"),
  ]).then(([m]) => m.default);
  return katexPromise;
}

const MathBlock: Component<{ node: MdNode }> = (props) => {
  const [html, setHtml] = createSignal<string>();
  const [error, setError] = createSignal<string>();
  const display = () => props.node.info === "display";
  let seq = 0;

  createEffect(
    on([() => props.node.literal ?? "", display], async ([tex, displayMode]) => {
      const id = ++seq;
      try {
        const katex = await loadKatex();
        const out = katex.renderToString(tex, {
          displayMode,
          throwOnError: false,
          output: "htmlAndMathml",
          trust: false, // documents are untrusted
          strict: "ignore",
        });
        if (id !== seq) return;
        setHtml(out);
        setError(undefined);
      } catch (e) {
        if (id !== seq) return;
        setError(e instanceof Error ? e.message : String(e));
        setHtml(undefined);
      }
    })
  );

  const raw = () => props.node.literal ?? "";

  return (
    <Show
      when={!error()}
      fallback={
        <Show
          when={display()}
          fallback={
            <span
              class="font-mono text-[0.9em] text-red-700 underline decoration-red-500 decoration-wavy dark:text-red-300"
              title={error()}
            >
              {raw()}
            </span>
          }
        >
          <div
            class="my-4 overflow-x-auto text-center font-mono text-[0.9em] text-red-700 underline decoration-red-500 decoration-wavy dark:text-red-300"
            title={error()}
          >
            {raw()}
          </div>
        </Show>
      }
    >
      <Show
        when={html() !== undefined}
        fallback={
          <Show
            when={display()}
            fallback={<span class="font-mono text-[0.9em] text-gray-500 dark:text-gray-400">{raw()}</span>}
          >
            <div class="my-4 overflow-x-auto text-center font-mono text-[0.9em] text-gray-500 dark:text-gray-400">
              {raw()}
            </div>
          </Show>
        }
      >
        <Show
          when={display()}
          fallback={<span class="math-inline" innerHTML={html()} />}
        >
          {/* KaTeX uses currentColor, so it follows the surrounding prose colour in both modes */}
          <div class="math-display my-4 overflow-x-auto overflow-y-hidden text-center" innerHTML={html()} />
        </Show>
      </Show>
    </Show>
  );
};

export default MathBlock;
