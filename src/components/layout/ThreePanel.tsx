import { Component, Show } from "solid-js";
import LeftSidebar from "./LeftSidebar";
import ContentArea from "./ContentArea";
import RightSidebar from "./RightSidebar";
import StatusBar from "./StatusBar";
import ResizeHandle from "./ResizeHandle";
import {
  leftPanelWidth,
  setLeftPanelWidth,
  rightPanelWidth,
  setRightPanelWidth,
  showLeftPanel,
  showRightPanel,
} from "../../stores/app";

const MIN_PANEL_WIDTH = 120;
const MAX_PANEL_WIDTH = 600;

function clampWidth(width: number): number {
  return Math.max(MIN_PANEL_WIDTH, Math.min(MAX_PANEL_WIDTH, width));
}

const ThreePanel: Component = () => {
  return (
    <div class="flex h-screen flex-col bg-white text-gray-900 dark:bg-gray-950 dark:text-gray-100">
      <div class="flex min-h-0 flex-1">
        <Show when={showLeftPanel()}>
          <div
            class="shrink-0"
            style={{ width: `${leftPanelWidth()}px` }}
          >
            <LeftSidebar />
          </div>
          <ResizeHandle
            side="left"
            onResize={(delta) =>
              setLeftPanelWidth(clampWidth(leftPanelWidth() + delta))
            }
          />
        </Show>
        <ContentArea />
        <Show when={showRightPanel()}>
          <ResizeHandle
            side="right"
            onResize={(delta) =>
              setRightPanelWidth(clampWidth(rightPanelWidth() + delta))
            }
          />
          <div
            class="shrink-0"
            style={{ width: `${rightPanelWidth()}px` }}
          >
            <RightSidebar />
          </div>
        </Show>
      </div>
      <StatusBar />
    </div>
  );
};

export default ThreePanel;
