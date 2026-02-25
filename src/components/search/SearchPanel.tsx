import { Component, For, Show } from "solid-js";
import { searchQuery, searchResults } from "../../stores/app";
import { performSearch, openDocument, toggleSearchMode } from "../../stores/actions";
import type { SearchMatch } from "../../lib/types";

const HighlightedLine: Component<{ match: SearchMatch }> = (props) => {
  const before = () => props.match.line_text.slice(0, props.match.col_start);
  const matched = () => props.match.line_text.slice(props.match.col_start, props.match.col_end);
  const after = () => props.match.line_text.slice(props.match.col_end);

  return (
    <span class="text-xs text-gray-600 dark:text-gray-400">
      {before()}
      <mark class="bg-yellow-200 text-yellow-900 dark:bg-yellow-800 dark:text-yellow-100">
        {matched()}
      </mark>
      {after()}
    </span>
  );
};

const SearchPanel: Component = () => {
  let inputRef: HTMLInputElement | undefined;

  const handleInput = (e: InputEvent) => {
    const target = e.target as HTMLInputElement;
    performSearch(target.value);
  };

  const handleClear = () => {
    performSearch("");
    inputRef?.focus();
  };

  const handleResultClick = (path: string) => {
    openDocument(path);
    toggleSearchMode();
  };

  return (
    <div class="flex h-full flex-col">
      <div class="relative px-2 pb-2">
        <svg
          class="pointer-events-none absolute left-4 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
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
          placeholder="Search files..."
          value={searchQuery()}
          onInput={handleInput}
          class="w-full rounded border border-gray-300 bg-white py-1.5 pl-8 pr-8 text-sm text-gray-900 placeholder-gray-400 focus:border-blue-500 focus:outline-none dark:border-gray-700 dark:bg-gray-900 dark:text-gray-100 dark:placeholder-gray-500 dark:focus:border-blue-400"
          autofocus
        />
        <Show when={searchQuery()}>
          <button
            class="absolute right-4 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
            onClick={handleClear}
          >
            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </Show>
      </div>

      <div class="flex-1 overflow-y-auto px-2">
        <Show
          when={searchQuery().trim()}
          fallback={
            <p class="text-sm text-gray-400 dark:text-gray-500">
              Type to search across all files.
            </p>
          }
        >
          <Show
            when={searchResults().length > 0}
            fallback={
              <p class="text-sm text-gray-400 dark:text-gray-500">No matches found.</p>
            }
          >
            <ul class="space-y-3">
              <For each={searchResults()}>
                {(result) => (
                  <li>
                    <button
                      class="w-full text-left"
                      onClick={() => handleResultClick(result.path)}
                    >
                      <div class="text-sm font-medium text-gray-800 hover:text-blue-600 dark:text-gray-200 dark:hover:text-blue-400">
                        {result.name}
                      </div>
                    </button>
                    <ul class="mt-1 space-y-0.5">
                      <For each={result.matches}>
                        {(match) => (
                          <li>
                            <button
                              class="flex w-full items-start gap-2 rounded px-1 py-0.5 text-left hover:bg-gray-100 dark:hover:bg-gray-800"
                              onClick={() => handleResultClick(result.path)}
                            >
                              <span class="shrink-0 text-xs tabular-nums text-gray-400">
                                {match.line_number}
                              </span>
                              <span class="min-w-0 truncate">
                                <HighlightedLine match={match} />
                              </span>
                            </button>
                          </li>
                        )}
                      </For>
                    </ul>
                  </li>
                )}
              </For>
            </ul>
          </Show>
        </Show>
      </div>
    </div>
  );
};

export default SearchPanel;
