/** CSS Custom Highlight API helpers for in-file search. */

const HIGHLIGHT_ALL = "search-match";
const HIGHLIGHT_CURRENT = "search-match-current";

/**
 * Walk all text nodes under `root` and find case-insensitive matches for `query`.
 * Returns an array of Range objects.
 */
export function findTextRanges(root: Node, query: string): Range[] {
  if (!query) return [];

  const lowerQuery = query.toLowerCase();
  const ranges: Range[] = [];
  const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT);

  let node: Text | null;
  while ((node = walker.nextNode() as Text | null)) {
    const text = node.textContent?.toLowerCase();
    if (!text) continue;

    let idx = 0;
    while ((idx = text.indexOf(lowerQuery, idx)) !== -1) {
      const range = document.createRange();
      range.setStart(node, idx);
      range.setEnd(node, idx + query.length);
      ranges.push(range);
      idx += query.length;
    }
  }

  return ranges;
}

/**
 * Apply CSS Custom Highlights for all matches and the current active match.
 */
export function applyHighlights(ranges: Range[], currentIndex: number): void {
  // @ts-expect-error -- CSS Custom Highlight API not yet in TS lib
  const HighlightCtor = globalThis.Highlight;
  if (!HighlightCtor || !CSS.highlights) return;

  if (ranges.length === 0) {
    clearHighlights();
    return;
  }

  // All matches highlight
  // @ts-expect-error -- CSS Custom Highlight API
  CSS.highlights.set(HIGHLIGHT_ALL, new HighlightCtor(...ranges));

  // Current match highlight
  if (ranges[currentIndex]) {
    // @ts-expect-error -- CSS Custom Highlight API
    CSS.highlights.set(HIGHLIGHT_CURRENT, new HighlightCtor(ranges[currentIndex]));
  }
}

/**
 * Remove all search highlights.
 */
export function clearHighlights(): void {
  // @ts-expect-error -- CSS Custom Highlight API
  CSS.highlights?.delete(HIGHLIGHT_ALL);
  // @ts-expect-error -- CSS Custom Highlight API
  CSS.highlights?.delete(HIGHLIGHT_CURRENT);
}
