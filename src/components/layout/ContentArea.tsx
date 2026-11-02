import { Component, Show, onCleanup, onMount, createEffect, createMemo } from "solid-js";
import { ast, setActiveTocId, findBarOpen, setFindBarOpen, zoom, contentWidth, setIsFullscreen, showSettings } from "../../stores/app";
import { updateConfig } from "../../stores/actions";
import { getCurrentWindow } from "@tauri-apps/api/window";
import MdRenderer from "../markdown/MdRenderer";
import FindBar from "../search/FindBar";

function isEditable(el: Element | null): boolean {
  return (
    el instanceof HTMLInputElement ||
    el instanceof HTMLTextAreaElement ||
    el instanceof HTMLSelectElement ||
    (el instanceof HTMLElement && el.isContentEditable)
  );
}

// Keys that scroll the document when nothing else has focus
const SCROLL_KEYS = new Set(["PageDown", "PageUp", "Home", "End", "ArrowDown", "ArrowUp", " "]);

const ContentArea: Component = () => {
  let contentRef: HTMLElement | undefined;
  let docRef: HTMLDivElement | undefined;
  let observer: IntersectionObserver | undefined;

  // Give the document keyboard focus so Page Down etc. scroll it without a
  // click first. Typing in an input (search, find bar) keeps its focus.
  const focusContent = () => {
    if (contentRef && !isEditable(document.activeElement)) {
      contentRef.focus({ preventScroll: true });
    }
  };

  // Scroll the document for a scroll key pressed while focus is on <body>,
  // e.g. after the find bar or settings closed and took focus with them.
  const scrollByKey = (e: KeyboardEvent): boolean => {
    const el = contentRef;
    if (!el || e.ctrlKey || e.metaKey || e.altKey || !SCROLL_KEYS.has(e.key)) return false;
    const page = Math.max(40, el.clientHeight - 40);
    switch (e.key) {
      case "PageDown": el.scrollBy({ top: page }); break;
      case "PageUp": el.scrollBy({ top: -page }); break;
      case " ": el.scrollBy({ top: e.shiftKey ? -page : page }); break;
      case "ArrowDown": el.scrollBy({ top: 40 }); break;
      case "ArrowUp": el.scrollBy({ top: -40 }); break;
      case "Home": el.scrollTo({ top: 0 }); break;
      case "End": el.scrollTo({ top: el.scrollHeight }); break;
    }
    return true;
  };

  createEffect(() => {
    const currentAst = ast();

    // Clean up previous observer
    observer?.disconnect();

    if (!currentAst || !contentRef) return;

    // Wait for DOM to render
    requestAnimationFrame(() => {
      focusContent();

      const headings = contentRef!.querySelectorAll<HTMLElement>(
        "h1[id], h2[id], h3[id], h4[id], h5[id], h6[id]"
      );

      if (headings.length === 0) return;

      observer = new IntersectionObserver(
        (entries) => {
          for (const entry of entries) {
            if (entry.isIntersecting) {
              setActiveTocId(entry.target.id);
            }
          }
        },
        {
          root: contentRef,
          rootMargin: "0px 0px -80% 0px",
          threshold: 0,
        }
      );

      headings.forEach((h) => observer!.observe(h));
    });
  });

  const widthClass = createMemo(() => {
    switch (contentWidth()) {
      case "fit": return "max-w-none";
      case "a4": return "max-w-[210mm]";
      default: return "max-w-3xl";
    }
  });

  const handleKeyDown = (e: KeyboardEvent) => {
    const active = document.activeElement;
    // Not while the settings dialog covers the document
    if ((!active || active === document.body) && !showSettings() && scrollByKey(e)) {
      e.preventDefault();
      focusContent();
      return;
    }
    // Ctrl+A selects only the document body, not sidebars/status bar.
    // Text inputs (FindBar, search, settings) keep their native select-all.
    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && e.key.toLowerCase() === "a") {
      if (!isEditable(e.target as Element | null) && docRef) {
        e.preventDefault();
        const range = document.createRange();
        range.selectNodeContents(docRef);
        const sel = window.getSelection();
        sel?.removeAllRanges();
        sel?.addRange(range);
      }
    }
    if (e.ctrlKey && !e.shiftKey && e.key === "f") {
      e.preventDefault();
      setFindBarOpen(!findBarOpen());
    }
    if (e.ctrlKey && (e.key === "=" || e.key === "+")) {
      e.preventDefault();
      updateConfig({ zoom: Math.min(300, zoom() + 10) });
    }
    if (e.ctrlKey && e.key === "-") {
      e.preventDefault();
      updateConfig({ zoom: Math.max(25, zoom() - 10) });
    }
    if (e.ctrlKey && e.key === "0") {
      e.preventDefault();
      updateConfig({ zoom: 100 });
    }
    if (e.key === "F11") {
      e.preventDefault();
      const win = getCurrentWindow();
      win.isFullscreen().then((fs) => {
        win.setFullscreen(!fs);
        setIsFullscreen(!fs);
      });
    }
  };

  onMount(() => {
    window.addEventListener("keydown", handleKeyDown);
    focusContent();
  });
  onCleanup(() => {
    observer?.disconnect();
    window.removeEventListener("keydown", handleKeyDown);
  });

  return (
    <div class="flex min-w-0 flex-1 flex-col">
      <main ref={contentRef} tabIndex={-1} class="min-w-0 flex-1 overflow-y-auto p-8 outline-none">
        <div ref={docRef} class={`mx-auto ${widthClass()}`} style={{ zoom: `${zoom()}%` }}>
          <Show
            when={ast()}
            fallback={
              <div>
                <h1 class="mb-4 text-3xl font-bold">Welcome to MarkRight</h1>
                <p class="text-gray-600 dark:text-gray-400">
                  Open a folder containing Markdown files to get started.
                </p>
              </div>
            }
          >
            <MdRenderer node={ast()!} />
          </Show>
        </div>
      </main>
      <FindBar contentRef={contentRef} />
    </div>
  );
};

export default ContentArea;
