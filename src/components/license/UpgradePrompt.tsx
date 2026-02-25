import { Component } from "solid-js";
import { upgradeFeatureName, setShowUpgradePrompt, setShowSettings } from "../../stores/app";
import { openUrl } from "@tauri-apps/plugin-opener";

const UpgradePrompt: Component = () => {
  const handleDonate = () => {
    openUrl("https://polar.sh/cybericius");
  };

  const handleHaveKey = () => {
    setShowUpgradePrompt(false);
    setShowSettings(true);
  };

  return (
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div class="mx-4 w-full max-w-md rounded-lg bg-white p-6 shadow-xl dark:bg-gray-900">
        <h2 class="mb-2 text-lg font-semibold text-gray-900 dark:text-gray-100">
          {upgradeFeatureName()} is a Pro Feature
        </h2>
        <p class="mb-4 text-sm text-gray-600 dark:text-gray-400">
          This feature is available to donors who support MarkRight's development.
        </p>
        <div class="mb-4 rounded bg-gray-50 p-3 text-sm text-gray-700 dark:bg-gray-800 dark:text-gray-300">
          <p class="mb-2 font-medium">How to unlock:</p>
          <ol class="list-inside list-decimal space-y-1">
            <li>
              Donate at{" "}
              <button
                onClick={handleDonate}
                class="font-medium text-blue-600 underline hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
              >
                polar.sh/cybericius
              </button>
            </li>
            <li>You'll receive a license key by email</li>
            <li>Paste it in Settings to activate</li>
          </ol>
        </div>
        <div class="flex justify-end gap-2">
          <button
            class="rounded-md px-4 py-2 text-sm font-medium text-blue-600 hover:bg-blue-50 dark:text-blue-400 dark:hover:bg-gray-800"
            onClick={handleHaveKey}
          >
            I have a key
          </button>
          <button
            class="rounded-md bg-gray-200 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600"
            onClick={() => setShowUpgradePrompt(false)}
          >
            Maybe Later
          </button>
        </div>
      </div>
    </div>
  );
};

export default UpgradePrompt;
