import { Component, createEffect } from "solid-js";
import { currentPath } from "../../stores/app";
import { resolveImageSrc } from "../../lib/paths";
import { followLink, isExternalUrl } from "./MdLink";

/**
 * Parse document HTML (already sanitized by the backend) into DOM nodes.
 * Relative `<img src>` values are rewritten to load from the document's
 * folder, and external links open in the browser.
 */
export function htmlNodes(html: string): Node[] {
  const tpl = document.createElement("template");
  tpl.innerHTML = html;
  for (const img of tpl.content.querySelectorAll("img")) {
    const src = img.getAttribute("src");
    if (src) img.src = resolveImageSrc(currentPath(), src);
  }
  for (const a of tpl.content.querySelectorAll("a")) {
    if (isExternalUrl(a.getAttribute("href") ?? "")) a.target = "_blank";
  }
  return Array.from(tpl.content.childNodes);
}

/** Click handler for a container of document HTML: links behave like Markdown links. */
export function onHtmlLinkClick(e: MouseEvent, container: Element | undefined) {
  const a = (e.target as Element).closest("a");
  if (a && container?.contains(a)) followLink(e, a.getAttribute("href") ?? "");
}

/** Raw HTML from the document, in its own wrapper element. */
const RawHtml: Component<{ html: string; inline?: boolean }> = (props) => {
  let el: HTMLElement | undefined;

  createEffect(() => {
    if (!el) return;
    el.replaceChildren(...htmlNodes(props.html));
  });

  const onClick = (e: MouseEvent) => onHtmlLinkClick(e, el);

  return props.inline ? (
    <span ref={(e) => (el = e)} onClick={onClick} />
  ) : (
    <div ref={(e) => (el = e)} onClick={onClick} />
  );
};

export default RawHtml;
