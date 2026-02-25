import { Component, onMount, onCleanup, createSignal } from "solid-js";
import { Show } from "solid-js/web";
import { openUrl } from "@tauri-apps/plugin-opener";
import {
  setShowSettings,
  fontFamilyUi,
  fontSizeUi,
  fontFamilyContent,
  fontSizeContent,
  lineHeightContent,
  leftPanelWidth,
  rightPanelWidth,
  showLeftPanel,
  showRightPanel,
  theme,
  zoom,
  contentWidth,
  licenseStatus,
} from "../../stores/app";
import { updateConfig, activateLicense } from "../../stores/actions";

const SettingsPanel: Component = () => {
  const onKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Escape") setShowSettings(false);
  };

  onMount(() => document.addEventListener("keydown", onKeyDown));
  onCleanup(() => document.removeEventListener("keydown", onKeyDown));

  return (
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/40"
      onClick={(e) => {
        if (e.target === e.currentTarget) setShowSettings(false);
      }}
    >
      <div class="w-full max-w-lg rounded-lg border border-gray-200 bg-white p-6 shadow-xl dark:border-gray-700 dark:bg-gray-900">
        <div class="mb-6 flex items-center justify-between">
          <h2 class="text-lg font-semibold">Settings</h2>
          <button
            class="rounded p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
            onClick={() => setShowSettings(false)}
          >
            ✕
          </button>
        </div>

        {/* View */}
        <Section title="View">
          <Row label="Zoom">
            <RangeInput
              value={zoom()}
              min={25}
              max={300}
              step={5}
              unit="%"
              onChange={(v) => updateConfig({ zoom: v })}
            />
          </Row>
          <Row label="Content Width">
            <div class="flex gap-3">
              {(["default", "fit", "a4"] as const).map((w) => (
                <label class="flex cursor-pointer items-center gap-1.5 text-sm">
                  <input
                    type="radio"
                    name="content-width"
                    checked={contentWidth() === w}
                    onChange={() => updateConfig({ content_width: w })}
                    class="accent-blue-500"
                  />
                  {w === "default" ? "Default" : w === "fit" ? "Fit Width" : "A4"}
                </label>
              ))}
            </div>
          </Row>
        </Section>

        {/* Content Font */}
        <Section title="Content Font">
          <Row label="Family">
            <input
              type="text"
              value={fontFamilyContent()}
              onInput={(e) => updateConfig({ font_family_content: e.currentTarget.value })}
              class="w-48 rounded border border-gray-300 bg-transparent px-2 py-1 text-sm dark:border-gray-600"
              list="font-suggestions-content"
            />
            <datalist id="font-suggestions-content">
              <option value="serif" />
              <option value="sans-serif" />
              <option value="Georgia" />
              <option value="Palatino" />
              <option value="Times New Roman" />
              <option value="Literata" />
            </datalist>
          </Row>
          <Row label="Size">
            <RangeInput
              value={fontSizeContent()}
              min={12}
              max={24}
              step={0.5}
              unit="px"
              onChange={(v) => updateConfig({ font_size_content: v })}
            />
          </Row>
          <Row label="Line Height">
            <RangeInput
              value={lineHeightContent()}
              min={1.2}
              max={2.0}
              step={0.05}
              onChange={(v) => updateConfig({ line_height_content: v })}
            />
          </Row>
        </Section>

        {/* UI Font */}
        <Section title="UI Font">
          <Row label="Family">
            <input
              type="text"
              value={fontFamilyUi()}
              onInput={(e) => updateConfig({ font_family_ui: e.currentTarget.value })}
              class="w-48 rounded border border-gray-300 bg-transparent px-2 py-1 text-sm dark:border-gray-600"
              list="font-suggestions-ui"
            />
            <datalist id="font-suggestions-ui">
              <option value="system-ui" />
              <option value="sans-serif" />
              <option value="Inter" />
              <option value="Segoe UI" />
              <option value="Helvetica Neue" />
            </datalist>
          </Row>
          <Row label="Size">
            <RangeInput
              value={fontSizeUi()}
              min={11}
              max={16}
              step={0.5}
              unit="px"
              onChange={(v) => updateConfig({ font_size_ui: v })}
            />
          </Row>
        </Section>

        {/* Panels */}
        <Section title="Panels">
          <Row label="Left Panel Width">
            <RangeInput
              value={leftPanelWidth()}
              min={120}
              max={600}
              step={1}
              unit="px"
              onChange={(v) => updateConfig({ left_panel_width: v })}
            />
          </Row>
          <Row label="Right Panel Width">
            <RangeInput
              value={rightPanelWidth()}
              min={120}
              max={600}
              step={1}
              unit="px"
              onChange={(v) => updateConfig({ right_panel_width: v })}
            />
          </Row>
          <Row label="Show Left Panel">
            <Toggle
              checked={showLeftPanel()}
              onChange={(v) => updateConfig({ show_left_panel: v })}
            />
          </Row>
          <Row label="Show Right Panel">
            <Toggle
              checked={showRightPanel()}
              onChange={(v) => updateConfig({ show_right_panel: v })}
            />
          </Row>
        </Section>

        {/* Theme */}
        <Section title="Theme">
          <div class="flex gap-3">
            {(["light", "dark", "system"] as const).map((t) => (
              <label class="flex cursor-pointer items-center gap-1.5 text-sm">
                <input
                  type="radio"
                  name="theme"
                  checked={theme() === t}
                  onChange={() => updateConfig({ theme: t })}
                  class="accent-blue-500"
                />
                {t.charAt(0).toUpperCase() + t.slice(1)}
              </label>
            ))}
          </div>
        </Section>

        {/* License */}
        <Section title="License">
          <LicenseSection />
        </Section>
      </div>
    </div>
  );
};

const LicenseSection: Component = () => {
  const [keyInput, setKeyInput] = createSignal("");
  const [message, setMessage] = createSignal<{ text: string; ok: boolean } | null>(null);
  const [activating, setActivating] = createSignal(false);

  const handleActivate = async () => {
    const key = keyInput().trim();
    if (!key) return;
    setActivating(true);
    setMessage(null);
    const ok = await activateLicense(key);
    setActivating(false);
    if (ok) {
      setMessage({ text: "License activated successfully!", ok: true });
      setKeyInput("");
    } else {
      setMessage({ text: "Invalid license key. Please check and try again.", ok: false });
    }
  };

  return (
    <div>
      <Show
        when={licenseStatus().valid}
        fallback={
          <div class="space-y-2">
            <p class="text-sm text-gray-500 dark:text-gray-400">
              Free tier —{" "}
              <button
                onClick={() => openUrl("https://buy.polar.sh/polar_cl_US7q3B0057nkiNtmToO7TegiQc6b6EMUeDD5k1ip2kD")}
                class="text-blue-600 underline hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
              >
                Get a Pro license
              </button>
            </p>
            <textarea
              value={keyInput()}
              onInput={(e) => setKeyInput(e.currentTarget.value)}
              placeholder="Paste your license key here..."
              rows={3}
              class="w-full rounded border border-gray-300 bg-transparent px-2 py-1.5 font-mono text-xs dark:border-gray-600"
            />
            <div class="flex items-center gap-3">
              <button
                onClick={handleActivate}
                disabled={activating() || !keyInput().trim()}
                class="rounded-md bg-blue-600 px-3 py-1.5 text-xs font-medium text-white hover:bg-blue-700 disabled:opacity-50"
              >
                {activating() ? "Activating..." : "Activate"}
              </button>
              <Show when={message()}>
                {(msg) => (
                  <span class={`text-xs ${msg().ok ? "text-green-600 dark:text-green-400" : "text-red-600 dark:text-red-400"}`}>
                    {msg().text}
                  </span>
                )}
              </Show>
            </div>
          </div>
        }
      >
        <div class="space-y-1 text-sm">
          <p class="text-green-600 dark:text-green-400">
            Licensed ({licenseStatus().tier})
          </p>
          <p class="text-gray-500 dark:text-gray-400">{licenseStatus().email}</p>
        </div>
      </Show>
    </div>
  );
};

const Section: Component<{ title: string; children: any }> = (props) => (
  <div class="mb-5">
    <h3 class="mb-2 text-xs font-semibold uppercase tracking-wide text-gray-400">
      {props.title}
    </h3>
    <div class="space-y-2">{props.children}</div>
  </div>
);

const Row: Component<{ label: string; children: any }> = (props) => (
  <div class="flex items-center justify-between">
    <span class="text-sm text-gray-600 dark:text-gray-300">{props.label}</span>
    {props.children}
  </div>
);

const RangeInput: Component<{
  value: number;
  min: number;
  max: number;
  step: number;
  unit?: string;
  onChange: (value: number) => void;
}> = (props) => (
  <div class="flex items-center gap-2">
    <input
      type="range"
      min={props.min}
      max={props.max}
      step={props.step}
      value={props.value}
      onInput={(e) => props.onChange(parseFloat(e.currentTarget.value))}
      class="w-32 accent-blue-500"
    />
    <span class="w-14 text-right text-xs text-gray-500">
      {Number.isInteger(props.step) ? props.value : props.value.toFixed(props.step < 0.1 ? 2 : 1)}
      {props.unit ?? ""}
    </span>
  </div>
);

const Toggle: Component<{
  checked: boolean;
  onChange: (value: boolean) => void;
}> = (props) => (
  <button
    class={`relative h-5 w-9 rounded-full transition-colors ${props.checked ? "bg-blue-500" : "bg-gray-300 dark:bg-gray-600"}`}
    onClick={() => props.onChange(!props.checked)}
  >
    <span
      class={`absolute top-0.5 h-4 w-4 rounded-full bg-white shadow transition-transform ${props.checked ? "left-[18px]" : "left-0.5"}`}
    />
  </button>
);

export default SettingsPanel;
