import { Component, JSX, Match, Switch, createResource } from "solid-js";
import type { MdNode } from "../../lib/types";
import { currentPath, tree } from "../../stores/app";
import { openDocument } from "../../stores/actions";
import { openLinkedFile, resolveWikilink } from "../../lib/tauri";
import { resolveImageSrc } from "../../lib/paths";

const IMAGE_EXT = /\.(png|jpe?g|gif|webp|avif|svg|bmp)$/i;
const MARKDOWN_EXT = /\.(md|markdown|mdown|mkd)$/i;

// One lookup per (document, target) pair; documents often repeat links.
// Dropped whenever the folder tree changes, so new files get found.
const cache = new Map<string, Promise<string | null>>();
let cacheTree: unknown;

function lookup(from: string, target: string): Promise<string | null> {
  if (tree() !== cacheTree) {
    cache.clear();
    cacheTree = tree();
  }
  const key = `${from}\n${target}`;
  let hit = cache.get(key);
  if (!hit) {
    hit = resolveWikilink(from, target).catch(() => null);
    cache.set(key, hit);
  }
  return hit;
}

function scrollToId(id: string | undefined) {
  if (id) document.getElementById(id)?.scrollIntoView({ behavior: "smooth" });
}

/**
 * Obsidian-style `[[Page#Heading|label]]` link and `![[image.png]]` embed.
 * `url` is the page part (empty = this document), `id` the heading slug,
 * `info === "embed"` for `![[...]]`.
 */
const WikiLink: Component<{ node: MdNode; children: JSX.Element }> = (props) => {
  const target = () => props.node.url ?? "";
  const [resolved] = createResource(
    () => (target() && currentPath() ? [currentPath()!, target()] as const : undefined),
    ([from, t]) => lookup(from, t)
  );

  // Obsidian image size: `![[img.png|300]]` or `![[img.png|300x200]]`
  const size = () => {
    const m = /^(\d+)(?:x(\d+))?$/.exec(props.node.children[0]?.literal?.trim() ?? "");
    return m ? [Number(m[1]), m[2] ? Number(m[2]) : undefined] : undefined;
  };

  const follow = (e: MouseEvent) => {
    e.preventDefault();
    if (!target()) {
      scrollToId(props.node.id);
      return;
    }
    const path = resolved();
    if (!path) return;
    if (MARKDOWN_EXT.test(path)) {
      openDocument(path).then(() => setTimeout(() => scrollToId(props.node.id), 50));
    } else {
      openLinkedFile(path).catch((err) => console.warn("Could not open link:", err));
    }
  };

  const link = () => (
    <a
      href={`#${props.node.id ?? ""}`}
      onClick={follow}
      title={target() || undefined}
      class="text-blue-600 underline hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
    >
      {props.children}
    </a>
  );

  return (
    <Switch fallback={link()}>
      {/* Still resolving: plain text, so nothing jumps when it settles */}
      <Match when={target() && resolved.loading}>
        <span>{props.children}</span>
      </Match>
      <Match when={target() && !resolved()}>
        <span
          class="cursor-help text-gray-500 underline decoration-dashed dark:text-gray-400"
          title={`Not found in this folder: ${target()}`}
        >
          {props.children}
        </span>
      </Match>
      <Match when={props.node.info === "embed" && IMAGE_EXT.test(resolved() ?? "")}>
        <img
          src={resolveImageSrc(currentPath(), resolved()!)}
          alt={target()}
          width={size()?.[0]}
          height={size()?.[1]}
        />
      </Match>
      <Match when={props.node.info === "embed"}>
        {/* Note embeds (transclusion) aren't supported yet: show a link */}
        {link()}
      </Match>
    </Switch>
  );
};

export default WikiLink;
