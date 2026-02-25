import { Component } from "solid-js";

const ContentArea: Component = () => {
  return (
    <main class="min-w-0 flex-1 overflow-y-auto p-8">
      <div class="mx-auto max-w-3xl">
        <h1 class="mb-4 text-3xl font-bold">Welcome to MarkRight</h1>
        <p class="text-gray-600 dark:text-gray-400">
          Open a folder containing Markdown files to get started.
        </p>
      </div>
    </main>
  );
};

export default ContentArea;
