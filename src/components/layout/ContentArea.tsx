import { Component, Show, onCleanup, createEffect } from "solid-js";
import { ast, setActiveTocId } from "../../stores/app";
import MdRenderer from "../markdown/MdRenderer";

const ContentArea: Component = () => {
  let contentRef: HTMLElement | undefined;
  let observer: IntersectionObserver | undefined;

  createEffect(() => {
    const currentAst = ast();

    // Clean up previous observer
    observer?.disconnect();

    if (!currentAst || !contentRef) return;

    // Wait for DOM to render
    requestAnimationFrame(() => {
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

  onCleanup(() => observer?.disconnect());

  return (
    <main ref={contentRef} class="min-w-0 flex-1 overflow-y-auto p-8">
      <div class="mx-auto max-w-3xl">
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
  );
};

export default ContentArea;
