/**
 * Send a license key to the customer via Resend email API.
 */
export async function sendLicenseEmail(
  to: string,
  licenseKey: string,
  apiKey: string,
): Promise<void> {
  const response = await fetch("https://api.resend.com/emails", {
    method: "POST",
    headers: {
      Authorization: `Bearer ${apiKey}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      from: "MarkRight <license@complitask.com>",
      to: [to],
      subject: "Your MarkRight Pro License Key",
      html: licenseEmailHtml(licenseKey),
    }),
  });

  if (!response.ok) {
    const text = await response.text();
    throw new Error(`Resend API error (${response.status}): ${text}`);
  }
}

function licenseEmailHtml(key: string): string {
  return `
<div style="font-family: system-ui, sans-serif; max-width: 560px; margin: 0 auto; padding: 32px 16px;">
  <h1 style="font-size: 20px; margin-bottom: 8px;">Thank you for supporting MarkRight!</h1>
  <p style="color: #555; margin-bottom: 24px;">Your Pro license key is below. This unlocks cross-file search and all future Pro features.</p>

  <div style="background: #f5f5f5; border: 1px solid #ddd; border-radius: 8px; padding: 16px; margin-bottom: 24px;">
    <p style="font-size: 12px; color: #888; margin: 0 0 8px;">License Key</p>
    <code style="font-size: 11px; word-break: break-all; display: block; line-height: 1.4;">${key}</code>
  </div>

  <h2 style="font-size: 16px; margin-bottom: 12px;">How to activate</h2>
  <ol style="color: #555; padding-left: 20px; line-height: 1.8;">
    <li>Open MarkRight and go to <strong>Settings</strong> (gear icon or Ctrl+,)</li>
    <li>Scroll to the <strong>License</strong> section</li>
    <li>Paste your license key and click <strong>Activate</strong></li>
  </ol>

  <p style="color: #999; font-size: 12px; margin-top: 24px;">
    Alternatively, save the key as <code>license.key</code> in your config directory:<br/>
    Linux: <code>~/.config/markright/license.key</code><br/>
    macOS: <code>~/Library/Application Support/markright/license.key</code><br/>
    Windows: <code>%APPDATA%\\markright\\license.key</code>
  </p>
</div>`;
}
