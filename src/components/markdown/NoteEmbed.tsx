import { Component, For, JSX, Show, createContext, createResource, useContext } from "solid-js";
import type { MdNode } from "../../lib/types";
import { getDocument } from "../../lib/tauri";
import { codeTheme, currentPath } from "../../stores/app";
import MdRenderer from "./MdRenderer";

/** Notes currently being embedded, outermost first (cycle and depth guard). */
const EmbedChain = createContext<string[]>([]);
const MAX_DEPTH = 3;
let embedSeq = 0;

/**
 * The part of a note to show: the whole document, or one section, from the
 * heading with `id` up to the next heading of the same or a higher level.
 */
function section(doc: MdNode, id: string | undefined): MdNode[] | undefined {
  if (!id) return doc.children;
  const start = doc.children.findIndex((n) => n.type === "Heading" && n.id === id);
  if (start < 0) return undefined;
  const level = doc.children[start].level ?? 1;
  const end = doc.children.findIndex((n, i) => i > start && n.type === "Heading" && (n.level ?? 1) <= level);
  return doc.children.slice(start, end < 0 ? undefined : end);
}

/**
 * Copy embedded nodes so their DOM ids can't clash with the host page:
 * heading ids are dropped (the host TOC doesn't list embedded headings) and
 * footnote ids get a per-embed prefix.
 */
function isolate(nodes: MdNode[], prefix: string): MdNode[] {
  return nodes.map((n) => ({
    ...n,
    id:
      n.type === "Heading"
        ? undefined
        : n.type === "FootnoteReference" || n.type === "FootnoteDefinition"
          ? `${prefix}${n.id}`
          : n.id,
    children: isolate(n.children, prefix),
  }));
}

/** `![[Note]]` / `![[Note#Heading]]`: the note's content shown inline. */
const NoteEmbed: Component<{ path: string; headingId?: string; link: JSX.Element }> = (props) => {
  const chain = useContext(EmbedChain);
  // The host document counts as an ancestor: a note can't embed itself.
  const blocked = () => props.path === currentPath() || chain.includes(props.path) || chain.length >= MAX_DEPTH;
  const prefix = `embed${++embedSeq}-`;

  const [doc] = createResource(
    () => (blocked() ? undefined : props.path),
    (path) => getDocument(path, codeTheme()).then((d) => d.ast)
  );
  const content = () => {
    const d = doc();
    const part = d && section(d, props.headingId);
    return part && isolate(part, prefix);
  };

  return (
    <Show when={!blocked()} fallback={props.link}>
      <div class="note-embed my-4 rounded-md border border-gray-200 px-4 py-2 dark:border-gray-700">
        <div class="not-prose mb-1 text-xs font-medium text-gray-500 dark:text-gray-400">{props.link}</div>
        <Show
          when={content()}
          fallback={
            <div class="text-sm text-gray-500 dark:text-gray-400">
              {doc.loading ? "Loading…" : doc.error ? "Could not load this note." : "Section not found."}
            </div>
          }
        >
          {(nodes) => (
            <EmbedChain.Provider value={[...chain, props.path]}>
              <For each={nodes()}>{(c) => <MdRenderer node={c} />}</For>
            </EmbedChain.Provider>
          )}
        </Show>
      </div>
    </Show>
  );
};

export default NoteEmbed;
