import { Component, For, JSX, Show } from "solid-js";
import { Dynamic } from "solid-js/web";
import type { MdNode } from "../../lib/types";
import MdRenderer from "./MdRenderer";
import MdLink from "./MdLink";
import { htmlNodes, onHtmlLinkClick } from "./RawHtml";

/** Parse the sanitized empty element (`<details open></details>`) the backend sends. */
function parseShell(shell: string): { tag: string; attrs: Record<string, string> } | undefined {
  const tpl = document.createElement("template");
  tpl.innerHTML = shell;
  const el = tpl.content.firstElementChild;
  if (!el) return undefined;
  const attrs: Record<string, string> = {};
  for (const a of el.attributes) attrs[a.name] = a.value;
  return { tag: el.tagName.toLowerCase(), attrs };
}

/**
 * Spread props that set real attributes. Without `attr:`, Solid assigns
 * boolean properties such as `open` directly, and `open=""` would close
 * the element.
 */
function asAttributes(attrs: Record<string, string>): Record<string, string> {
  return Object.fromEntries(Object.entries(attrs).map(([k, v]) => [`attr:${k}`, v]));
}

/**
 * An HTML element from the document whose content is Markdown, e.g.
 * `<details>` with a Markdown body or `<kbd>Ctrl</kbd>`. HTML children are
 * placed as bare nodes, so `<summary>` stays a direct child of `<details>`.
 */
const HtmlElement: Component<{ node: MdNode }> = (props) => {
  const shell = () => parseShell(props.node.literal ?? "");
  let el: Element | undefined;

  const content = (): JSX.Element => (
    <For each={props.node.children}>
      {(c) =>
        c.type === "HtmlBlock" || c.type === "HtmlInline" ? htmlNodes(c.literal ?? "") : <MdRenderer node={c} />
      }
    </For>
  );

  return (
    <Show when={shell()} fallback={content()}>
      {(s) => (
        <Show
          when={s().tag !== "a"}
          fallback={
            <MdLink url={s().attrs.href ?? ""} title={s().attrs.title}>
              {content()}
            </MdLink>
          }
        >
          <Dynamic
            component={s().tag}
            {...asAttributes(s().attrs)}
            ref={(e: Element) => (el = e)}
            onClick={(e: MouseEvent) => onHtmlLinkClick(e, el)}
          >
            {content()}
          </Dynamic>
        </Show>
      )}
    </Show>
  );
};

export default HtmlElement;
