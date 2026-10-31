import { Component, For, JSX, Show } from "solid-js";
import type { MdNode } from "../../lib/types";
import MdRenderer from "./MdRenderer";

type Tone = "blue" | "cyan" | "green" | "purple" | "amber" | "orange" | "red" | "gray";

const TONE: Record<Tone, string> = {
  blue: "border-blue-500 bg-blue-50 text-blue-700 dark:bg-blue-950/40 dark:text-blue-300",
  cyan: "border-cyan-500 bg-cyan-50 text-cyan-700 dark:bg-cyan-950/40 dark:text-cyan-300",
  green: "border-green-600 bg-green-50 text-green-700 dark:bg-green-950/40 dark:text-green-300",
  purple: "border-purple-500 bg-purple-50 text-purple-700 dark:bg-purple-950/40 dark:text-purple-300",
  amber: "border-amber-500 bg-amber-50 text-amber-700 dark:bg-amber-950/40 dark:text-amber-300",
  orange: "border-orange-500 bg-orange-50 text-orange-700 dark:bg-orange-950/40 dark:text-orange-300",
  red: "border-red-500 bg-red-50 text-red-700 dark:bg-red-950/40 dark:text-red-300",
  gray: "border-gray-400 bg-gray-50 text-gray-600 dark:bg-gray-900/60 dark:text-gray-300",
};

// Icon shapes drawn in a 16x16 box with a 1.5 stroke. Functions, not
// elements: a Solid JSX element is a single DOM node and can't be reused.
const circle = () => <circle cx="8" cy="8" r="6.25" />;
const ICONS: Record<string, () => JSX.Element> = {
  info: () => (<>{circle()}<path d="M8 7.5v4M8 5h.01" /></>),
  bulb: () => (<><path d="M5.5 10.5a4 4 0 1 1 5 0c-.4.3-.5.8-.5 1.2V12H6v-.3c0-.4-.1-.9-.5-1.2Z" /><path d="M6.5 14h3" /></>),
  alert: () => (<>{circle()}<path d="M8 4.75v4M8 11h.01" /></>),
  triangle: () => (<><path d="M8 2 14.5 13.5h-13Z" /><path d="M8 6.5v3M8 11.5h.01" /></>),
  octagon: () => (<><path d="M5.5 1.75h5l3.75 3.75v5l-3.75 3.75h-5L1.75 10.5v-5Z" /><path d="M8 5v3.5M8 11h.01" /></>),
  check: () => (<>{circle()}<path d="m5.25 8.25 2 2 3.5-4" /></>),
  question: () => (<>{circle()}<path d="M6.25 6.25a1.85 1.85 0 0 1 3.6.6c0 1.25-1.85 1.5-1.85 2.65M8 11.25h.01" /></>),
  cross: () => (<>{circle()}<path d="m5.75 5.75 4.5 4.5M10.25 5.75l-4.5 4.5" /></>),
  list: () => (<path d="M3 4h10M3 8h10M3 12h6" />),
  quote: () => (<path d="M3.5 11.5c0-3 1-5 3-6M9.5 11.5c0-3 1-5 3-6M3.5 11.5h2.5v-2.5H3.5ZM9.5 11.5H12v-2.5H9.5Z" />),
};

const KIND: Record<string, { tone: Tone; icon: string }> = {
  note: { tone: "blue", icon: "info" },
  info: { tone: "blue", icon: "info" },
  todo: { tone: "blue", icon: "check" },
  abstract: { tone: "cyan", icon: "list" },
  tip: { tone: "green", icon: "bulb" },
  success: { tone: "green", icon: "check" },
  important: { tone: "purple", icon: "alert" },
  example: { tone: "purple", icon: "list" },
  warning: { tone: "amber", icon: "triangle" },
  question: { tone: "orange", icon: "question" },
  caution: { tone: "red", icon: "octagon" },
  danger: { tone: "red", icon: "octagon" },
  failure: { tone: "red", icon: "cross" },
  bug: { tone: "red", icon: "alert" },
  quote: { tone: "gray", icon: "quote" },
};

/** GitHub alert / Obsidian callout. `info` is the kind, `fold` makes it collapsible. */
const Alert: Component<{ node: MdNode }> = (props) => {
  const kind = () => KIND[props.node.info ?? ""] ?? KIND.note;
  const customTitle = () => props.node.children[0]?.type === "AlertTitle" ? props.node.children[0] : undefined;
  const body = () => (customTitle() ? props.node.children.slice(1) : props.node.children);

  const title = () => (
    <span class="not-prose flex items-center gap-2 font-semibold">
      <svg viewBox="0 0 16 16" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        {ICONS[kind().icon]()}
      </svg>
      <Show when={customTitle()} fallback={props.node.title}>
        {(t) => <For each={t().children}>{(c) => <MdRenderer node={c} />}</For>}
      </Show>
    </span>
  );
  const content = () => (
    <div class="alert-body text-gray-800 dark:text-gray-200 [&>*:first-child]:mt-2 [&>*:last-child]:mb-0">
      <For each={body()}>{(c) => <MdRenderer node={c} />}</For>
    </div>
  );

  return (
    <div class={`my-4 rounded-r-md border-l-4 px-4 py-3 ${TONE[kind().tone]}`} role="note">
      <Show when={props.node.fold} fallback={<>{title()}{content()}</>}>
        <details open={props.node.fold === "open"}>
          <summary class="cursor-pointer list-none [&::-webkit-details-marker]:hidden">{title()}</summary>
          {content()}
        </details>
      </Show>
    </div>
  );
};

export default Alert;
