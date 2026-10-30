import { Component, createEffect } from "solid-js";
import { currentPath } from "../../stores/app";
import { resolveImageSrc } from "../../lib/paths";

/**
 * Raw HTML from the document (already sanitized by the backend). Relative
 * `<img src>` values are rewritten so they load from the document's folder.
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
  });

  return props.inline ? <span ref={(e) => (el = e)} /> : <div ref={(e) => (el = e)} />;
};

export default RawHtml;
