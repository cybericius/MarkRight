import { Component, JSX } from "solid-js";
import { openDocument } from "../../stores/actions";
import { currentPath } from "../../stores/app";
import { resolvePath } from "../../lib/paths";
import { openLinkedFile } from "../../lib/tauri";

const MARKDOWN_EXT = /\.(md|markdown|mdown|mkd)$/i;

export function isExternalUrl(url: string): boolean {
  return url.startsWith("http://") || url.startsWith("https://");
}

function decodePath(path: string): string {
  try {
    return decodeURI(path);
  } catch {
    // Malformed escapes: use the path as written
    return path;
  }
}

/**
 * Handle a click on a document link (Markdown or raw HTML). Anchors scroll,
 * Markdown files open in the app, other local files go to the backend;
 * external links are left to the default handler.
 */
export function followLink(e: MouseEvent, url: string): void {
  // Anchor link within current document
  if (url.startsWith("#")) {
    e.preventDefault();
    const el = document.getElementById(url.slice(1));
    el?.scrollIntoView({ behavior: "smooth" });
    return;
  }

  // Absolute URLs with a scheme (http, mailto, ...) and blocked (empty) URLs
  if (!url || /^[a-z][a-z0-9+.-]+:/i.test(url)) return;

  e.preventDefault();
  const current = currentPath();
  const [rawPath, anchor] = url.split("#");
  const filePart = rawPath.split("?")[0];
  if (!current || !filePart) return;
  const resolved = resolvePath(current, decodePath(filePart));

  // Internal Markdown link
  if (MARKDOWN_EXT.test(filePart)) {
    openDocument(resolved).then(() => {
      if (anchor) {
        // Small delay to let the DOM render
        setTimeout(() => {
          const el = document.getElementById(anchor);
          el?.scrollIntoView({ behavior: "smooth" });
        }, 50);
      }
    });
    return;
  }

  // Other local files (PDFs, images, ...) open in their default app
  openLinkedFile(resolved).catch((err) => console.warn("Could not open link:", err));
}

const MdLink: Component<{
  url: string;
  title?: string;
  children: JSX.Element;
}> = (props) => {
  // External links open in default browser (Tauri handles this)
  const isExternal = () => isExternalUrl(props.url);

  return (
    <a
      // Unsafe URLs arrive empty; an empty href would reload the app
      href={props.url || undefined}
      title={props.title}
      onClick={(e) => followLink(e, props.url)}
      target={isExternal() ? "_blank" : undefined}
      rel={isExternal() ? "noopener noreferrer" : undefined}
      class="text-blue-600 underline hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
    >
      {props.children}
    </a>
  );
};

export default MdLink;
