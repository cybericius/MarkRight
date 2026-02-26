import { Component, For, Switch, Match, Show } from "solid-js";
import type { MdNode } from "../../lib/types";
import CodeBlock from "./CodeBlock";
import MdLink from "./MdLink";

const MdRenderer: Component<{ node: MdNode }> = (props) => {
  const children = () => (
    <For each={props.node.children}>
      {(child) => <MdRenderer node={child} />}
    </For>
  );

  return (
    <Switch fallback={children()}>
      <Match when={props.node.type === "Document"}>
        <div class="prose prose-gray max-w-none dark:prose-invert" style={{ "font-family": "var(--font-content)", "font-size": "var(--font-content-size)", "line-height": "var(--font-content-line-height)" }}>{children()}</div>
      </Match>

      <Match when={props.node.type === "Heading" && props.node.level === 1}>
        <h1 id={props.node.id}>{children()}</h1>
      </Match>
      <Match when={props.node.type === "Heading" && props.node.level === 2}>
        <h2 id={props.node.id}>{children()}</h2>
      </Match>
      <Match when={props.node.type === "Heading" && props.node.level === 3}>
        <h3 id={props.node.id}>{children()}</h3>
      </Match>
      <Match when={props.node.type === "Heading" && props.node.level === 4}>
        <h4 id={props.node.id}>{children()}</h4>
      </Match>
      <Match when={props.node.type === "Heading" && props.node.level === 5}>
        <h5 id={props.node.id}>{children()}</h5>
      </Match>
      <Match when={props.node.type === "Heading" && props.node.level === 6}>
        <h6 id={props.node.id}>{children()}</h6>
      </Match>

      <Match when={props.node.type === "Paragraph"}>
        <p>{children()}</p>
      </Match>

      <Match when={props.node.type === "Text"}>
        {props.node.literal ?? ""}
      </Match>

      <Match when={props.node.type === "SoftBreak"}>
        {" "}
      </Match>

      <Match when={props.node.type === "LineBreak"}>
        <br />
      </Match>

      <Match when={props.node.type === "ThematicBreak"}>
        <hr />
      </Match>

      <Match when={props.node.type === "BlockQuote"}>
        <blockquote>{children()}</blockquote>
      </Match>

      <Match when={props.node.type === "Code"}>
        <code>{props.node.literal ?? ""}</code>
      </Match>

      <Match when={props.node.type === "CodeBlock"}>
        <CodeBlock node={props.node} />
      </Match>

      <Match when={props.node.type === "HtmlBlock"}>
        <div innerHTML={props.node.literal ?? ""} />
      </Match>

      <Match when={props.node.type === "HtmlInline"}>
        <span innerHTML={props.node.literal ?? ""} />
      </Match>

      <Match when={props.node.type === "Emph"}>
        <em>{children()}</em>
      </Match>

      <Match when={props.node.type === "Strong"}>
        <strong>{children()}</strong>
      </Match>

      <Match when={props.node.type === "Strikethrough"}>
        <del>{children()}</del>
      </Match>

      <Match when={props.node.type === "Link"}>
        <MdLink url={props.node.url ?? ""} title={props.node.title}>
          {children()}
        </MdLink>
      </Match>

      <Match when={props.node.type === "Image"}>
        <img src={props.node.url ?? ""} alt="" title={props.node.title} />
      </Match>

      <Match when={props.node.type === "List" && props.node.list_type === "ordered"}>
        <ol start={props.node.start}>{children()}</ol>
      </Match>

      <Match when={props.node.type === "List" && props.node.list_type === "bullet"}>
        <ul>{children()}</ul>
      </Match>

      <Match when={props.node.type === "Item"}>
        <li>{children()}</li>
      </Match>

      <Match when={props.node.type === "TaskItem"}>
        <li class="flex items-start gap-2">
          <input
            type="checkbox"
            checked={props.node.checked ?? false}
            disabled
            class="mt-1.5"
          />
          <span>{children()}</span>
        </li>
      </Match>

      <Match when={props.node.type === "Table"}>
        <div class="overflow-x-auto">
          <table>{children()}</table>
        </div>
      </Match>

      <Match when={props.node.type === "TableRow" && props.node.header}>
        <thead>
          <tr>{children()}</tr>
        </thead>
      </Match>

      <Match when={props.node.type === "TableRow" && !props.node.header}>
        <tr>{children()}</tr>
      </Match>

      <Match when={props.node.type === "TableCell"}>
        <td>{children()}</td>
      </Match>
    </Switch>
  );
};

export default MdRenderer;
