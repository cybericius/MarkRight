import { Component, Show, createEffect, createSignal, on } from "solid-js";
import { isDark } from "../../stores/app";

// Lazy-load viz.js (Graphviz compiled to wasm): large, only for documents with graphs.
let vizPromise: Promise<Awaited<ReturnType<typeof import("@viz-js/viz").instance>>> | undefined;

function loadViz() {
  vizPromise ??= import("@viz-js/viz").then((m) => m.instance());
  return vizPromise;
}

/**
 * Parse Graphviz SVG output and strip anything active. DOT can attach links
 * (`URL=`, `href=`) to nodes; the document is untrusted, so links are removed
 * rather than followed. DOMParser never runs scripts.
 */
function sanitizeSvg(svgText: string): SVGSVGElement | undefined {
  const doc = new DOMParser().parseFromString(svgText, "image/svg+xml");
  const svg = doc.documentElement;
  if (!(svg instanceof SVGSVGElement)) return undefined;
  for (const el of svg.querySelectorAll("script, foreignObject")) el.remove();
  // Unwrap DOT links (`URL=`/`href=`) into plain groups: no navigation, and
  // no link styling from the surrounding prose.
  for (const a of svg.querySelectorAll("a")) {
    const g = doc.createElementNS("http://www.w3.org/2000/svg", "g");
    g.append(...a.childNodes);
    a.replaceWith(g);
  }
  for (const el of [svg, ...svg.querySelectorAll("*")]) {
    for (const attr of [...el.attributes]) {
      const name = attr.name.toLowerCase();
      if (name.startsWith("on") || name === "href" || name === "xlink:href" || name === "target") {
        el.removeAttribute(attr.name);
      }
    }
  }
  // Natural size, but shrink to fit narrow columns.
  svg.style.maxWidth = "100%";
  svg.style.height = "auto";
  return svg;
}

const GraphvizBlock: Component<{ source: string }> = (props) => {
  const [svg, setSvg] = createSignal<SVGSVGElement>();
  const [error, setError] = createSignal<string>();
  let latest = 0;

  createEffect(
    on([() => props.source, isDark], async ([source, dark]) => {
      const run = ++latest;
      try {
        const viz = await loadViz();
        // Defaults only: colours set in the DOT source still win.
        const ink = dark ? "#d1d5db" : "#1f2937";
        const result = viz.render(source, {
          format: "svg",
          graphAttributes: { bgcolor: "transparent", fontcolor: ink },
          nodeAttributes: { color: ink, fontcolor: ink },
          edgeAttributes: { color: ink, fontcolor: ink },
        });
        if (run !== latest) return;
        const out = result.status === "success" ? sanitizeSvg(result.output) : undefined;
        if (!out) {
          setError(result.errors.map((e) => e.message).join("\n") || "Could not render graph");
          setSvg(undefined);
          return;
        }
        setSvg(out);
        setError(undefined);
      } catch (e) {
        if (run !== latest) return;
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
          <div class="mb-2 font-semibold text-red-600 dark:text-red-400">Graphviz error</div>
          <pre class="overflow-x-auto whitespace-pre-wrap text-red-700 dark:text-red-300">{error()}</pre>
          <pre class="mt-2 overflow-x-auto text-gray-700 dark:text-gray-300">
            <code>{props.source}</code>
          </pre>
        </div>
      }
    >
      <div class="graphviz-block my-4 flex justify-center overflow-x-auto">{svg()}</div>
    </Show>
  );
};

export default GraphvizBlock;
