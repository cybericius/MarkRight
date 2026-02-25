import { Component } from "solid-js";

const ResizeHandle: Component<{
  onResize: (delta: number) => void;
  onDragEnd?: () => void;
  side: "left" | "right";
}> = (props) => {
  let startX = 0;

  const onMouseDown = (e: MouseEvent) => {
    e.preventDefault();
    startX = e.clientX;

    const onMouseMove = (e: MouseEvent) => {
      const delta = e.clientX - startX;
      startX = e.clientX;
      // For the left handle, dragging right = positive delta = wider left panel
      // For the right handle, dragging left = negative delta = wider right panel
      props.onResize(props.side === "left" ? delta : -delta);
    };

    const onMouseUp = () => {
      document.removeEventListener("mousemove", onMouseMove);
      document.removeEventListener("mouseup", onMouseUp);
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
      props.onDragEnd?.();
    };

    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);
    document.body.style.cursor = "col-resize";
    document.body.style.userSelect = "none";
  };

  return (
    <div
      class="w-1 shrink-0 cursor-col-resize bg-transparent transition-colors hover:bg-blue-400/50 active:bg-blue-500/50"
      onMouseDown={onMouseDown}
    />
  );
};

export default ResizeHandle;
