import { Component, createEffect, onCleanup } from "solid-js";
import {
  findBarOpen,
  setFindBarOpen,
  findQuery,
  setFindQuery,
  findMatchCount,
  setFindMatchCount,
  findCurrentIndex,
  setFindCurrentIndex,
  ast,
} from "../../stores/app";
import { findTextRanges, applyHighlights, clearHighlights } from "../../lib/highlight";

interface FindBarProps {
  contentRef: HTMLElement | undefined;
}

const FindBar: Component<FindBarProps> = (props) => {
  let inputRef: HTMLInputElement | undefined;
  let currentRanges: Range[] = [];

  // Recompute highlights when query or ast changes
  createEffect(() => {
    const query = findQuery();
    const _ast = ast(); // track reactivity on document changes

    if (!props.contentRef || !findBarOpen()) {
      clearHighlights();
      setFindMatchCount(0);
      setFindCurrentIndex(0);
      currentRanges = [];
      return;
    }

    requestAnimationFrame(() => {
      currentRanges = findTextRanges(props.contentRef!, query);
      setFindMatchCount(currentRanges.length);
      const idx = currentRanges.length > 0 ? 0 : 0;
      setFindCurrentIndex(idx);
      applyHighlights(currentRanges, idx);
      scrollToCurrent(idx);
    });
  });

  // Update highlight when currentIndex changes independently
  createEffect(() => {
    const idx = findCurrentIndex();
    if (currentRanges.length > 0) {
      applyHighlights(currentRanges, idx);
    }
  });

  onCleanup(() => clearHighlights());

  function scrollToCurrent(index: number) {
    const range = currentRanges[index];
    if (!range) return;
    const rect = range.getBoundingClientRect();
    const container = props.contentRef;
    if (!container) return;
    const containerRect = container.getBoundingClientRect();
    if (rect.top < containerRect.top || rect.bottom > containerRect.bottom) {
      const el = range.startContainer.parentElement;
      el?.scrollIntoView({ block: "center", behavior: "smooth" });
    }
  }

  function goNext() {
    if (currentRanges.length === 0) return;
    const next = (findCurrentIndex() + 1) % currentRanges.length;
    setFindCurrentIndex(next);
    scrollToCurrent(next);
  }

  function goPrev() {
    if (currentRanges.length === 0) return;
    const prev = (findCurrentIndex() - 1 + currentRanges.length) % currentRanges.length;
    setFindCurrentIndex(prev);
    scrollToCurrent(prev);
  }

  function close() {
    setFindBarOpen(false);
    setFindQuery("");
    clearHighlights();
    currentRanges = [];
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      close();
    } else if (e.key === "Enter" && e.shiftKey) {
      e.preventDefault();
      goPrev();
    } else if (e.key === "Enter") {
      e.preventDefault();
      goNext();
    }
  }

  // Auto-focus input when bar opens
  createEffect(() => {
    if (findBarOpen() && inputRef) {
      requestAnimationFrame(() => inputRef!.focus());
    }
  });

  return (
    <div
      class="flex items-center gap-2 border-t border-gray-200 bg-gray-50 px-4 py-1.5 dark:border-gray-800 dark:bg-gray-900"
      style={{ display: findBarOpen() ? "flex" : "none" }}
    >
      <svg class="h-4 w-4 shrink-0 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
        />
      </svg>
      <input
        ref={inputRef}
        type="text"
        class="min-w-0 flex-1 border-none bg-transparent text-sm outline-none placeholder:text-gray-400"
        placeholder="Find in document..."
        value={findQuery()}
        onInput={(e) => setFindQuery(e.currentTarget.value)}
        onKeyDown={handleKeyDown}
      />
      <span class="shrink-0 text-xs text-gray-500">
        {findMatchCount() > 0
          ? `${findCurrentIndex() + 1} / ${findMatchCount()}`
          : findQuery()
            ? "No matches"
            : ""}
      </span>
      <button
        class="rounded p-0.5 text-gray-500 hover:bg-gray-200 dark:hover:bg-gray-700"
        onClick={goPrev}
        title="Previous (Shift+Enter)"
      >
        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
        </svg>
      </button>
      <button
        class="rounded p-0.5 text-gray-500 hover:bg-gray-200 dark:hover:bg-gray-700"
        onClick={goNext}
        title="Next (Enter)"
      >
        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
      </button>
      <button
        class="rounded p-0.5 text-gray-500 hover:bg-gray-200 dark:hover:bg-gray-700"
        onClick={close}
        title="Close (Escape)"
      >
        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  );
};

export default FindBar;
