import { Component, JSX } from "solid-js";
import { openDocument } from "../../stores/actions";
import { currentPath } from "../../stores/app";
import { resolvePath } from "../../lib/paths";

const MdLink: Component<{
  url: string;
  title?: string;
  children: JSX.Element;
}> = (props) => {
  const handleClick = (e: MouseEvent) => {
    const url = props.url;

    // Anchor link within current document
    if (url.startsWith("#")) {
      e.preventDefault();
      const el = document.getElementById(url.slice(1));
      el?.scrollIntoView({ behavior: "smooth" });
      return;
    }

    // Internal .md link
    if (url.endsWith(".md") || url.includes(".md#")) {
      e.preventDefault();
      const [filePart, anchor] = url.split("#");
      const current = currentPath();
      if (current && filePart) {
        const resolved = resolvePath(current, filePart);
        openDocument(resolved).then(() => {
          if (anchor) {
            // Small delay to let the DOM render
            setTimeout(() => {
              const el = document.getElementById(anchor);
              el?.scrollIntoView({ behavior: "smooth" });
            }, 50);
          }
        });
      }
      return;
    }

    // External links open in default browser (Tauri handles this)
  };

  const isExternal = () => {
    const url = props.url;
    return url.startsWith("http://") || url.startsWith("https://");
  };

  return (
    <a
      href={props.url}
      title={props.title}
      onClick={handleClick}
      target={isExternal() ? "_blank" : undefined}
      rel={isExternal() ? "noopener noreferrer" : undefined}
      class="text-blue-600 underline hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
    >
      {props.children}
    </a>
  );
};

export default MdLink;
