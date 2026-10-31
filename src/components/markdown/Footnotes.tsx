import { Component, For, Show } from "solid-js";
import type { MdNode } from "../../lib/types";
import MdRenderer from "./MdRenderer";

/** Smooth-scroll to an element id without changing the URL hash. */
function jumpTo(e: MouseEvent, id: string) {
  e.preventDefault();
  document.getElementById(id)?.scrollIntoView({ behavior: "smooth", block: "center" });
}

const SUPERSCRIPT = ["⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹"];
const sup = (n: number) => String(n).split("").map((d) => SUPERSCRIPT[Number(d)]).join("");

/** Inline footnote reference: a superscript number linking to the definition. */
export const FootnoteRef: Component<{ node: MdNode }> = (props) => {
  const id = () => props.node.id ?? "";
  return (
    <sup id={`fnref-${id()}-${props.node.ref_num ?? 1}`} class="footnote-ref">
      <a
        href={`#fn-${id()}`}
        class="px-0.5 text-blue-600 no-underline hover:underline dark:text-blue-400"
        aria-label={`Footnote ${props.node.start ?? ""}`}
        onClick={(e) => jumpTo(e, `fn-${id()}`)}
      >
        {props.node.start}
      </a>
    </sup>
  );
};

const BackLinks: Component<{ id: string; count: number }> = (props) => {
  const refs = () => Array.from({ length: props.count }, (_, i) => i + 1);
  return (
    <For each={refs()}>
      {(n) => (
        <a
          href={`#fnref-${props.id}-${n}`}
          class="ml-1 text-blue-600 no-underline hover:underline dark:text-blue-400"
          aria-label={`Back to reference ${n}`}
          onClick={(e) => jumpTo(e, `fnref-${props.id}-${n}`)}
        >
          {n === 1 ? "↩" : `↩${sup(n)}`}
        </a>
      )}
    </For>
  );
};

/** One definition: back-links go inline at the end of a trailing paragraph. */
const Definition: Component<{ node: MdNode }> = (props) => {
  const id = () => props.node.id ?? "";
  const count = () => props.node.ref_count ?? 0;
  const last = () => props.node.children[props.node.children.length - 1];
  const inline = () => last()?.type === "Paragraph";
  const lead = () => (inline() ? props.node.children.slice(0, -1) : props.node.children);

  return (
    <li id={`fn-${id()}`} value={props.node.start} class="pl-1 [&>*:first-child]:mt-0 [&>*]:my-1">
      <For each={lead()}>{(c) => <MdRenderer node={c} />}</For>
      <Show when={inline()}>
        <p>
          <For each={last().children}>{(c) => <MdRenderer node={c} />}</For>
          <BackLinks id={id()} count={count()} />
        </p>
      </Show>
      <Show when={!inline() && count() > 0}>
        <p>
          <BackLinks id={id()} count={count()} />
        </p>
      </Show>
    </li>
  );
};

/** Footnotes block at the end of a document. Label is a div, not a heading, to stay out of the TOC. */
export const FootnoteSection: Component<{ node: MdNode }> = (props) => (
  <section class="footnotes mt-10 border-t border-gray-300 pt-3 text-sm text-gray-600 dark:border-gray-700 dark:text-gray-400">
    <div class="mb-2 text-xs font-semibold uppercase tracking-wide text-gray-500 dark:text-gray-500">Footnotes</div>
    <ol class="m-0 list-decimal pl-6">
      <For each={props.node.children}>{(d) => <Definition node={d} />}</For>
    </ol>
  </section>
);
