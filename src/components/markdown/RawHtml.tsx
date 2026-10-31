import { Component, createEffect } from "solid-js";
import { currentPath } from "../../stores/app";
import { resolveImageSrc } from "../../lib/paths";
import { followLink, isExternalUrl } from "./MdLink";

/**
 * Raw HTML from the document (already sanitized by the backend). Relative
 * `<img src>` values are rewritten so they load from the document's folder,
 * and links behave like Markdown links instead of navigating the app away.
 */
const RawHtml: Component<{ html: string; inline?: boolean }> = (props) => {
  let el: HTMLElement | undefined;

  createEffect(() => {
    if (!el) return;
    el.innerHTML = props.html;
    for (const img of el.querySelectorAll("img")) {
      const src = img.getAttribute("src");
      if (src) img.src = resolveImageSrc(currentPath(), src);
    }
    for (const a of el.querySelectorAll("a")) {
      if (isExternalUrl(a.getAttribute("href") ?? "")) a.target = "_blank";
    }
  });

  const onClick = (e: MouseEvent) => {
    const a = (e.target as Element).closest("a");
    if (a && el?.contains(a)) followLink(e, a.getAttribute("href") ?? "");
  };

  return props.inline ? (
    <span ref={(e) => (el = e)} onClick={onClick} />
  ) : (
    <div ref={(e) => (el = e)} onClick={onClick} />
  );
};

export default RawHtml;
