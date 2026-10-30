import { convertFileSrc } from "@tauri-apps/api/core";

/** Resolve a relative path against the directory of the current document. */
export function resolvePath(currentPath: string, relative: string): string {
  // Windows paths use backslashes and a drive prefix ("C:\docs\a.md").
  const current = currentPath.replace(/\\/g, "/");
  const rel = relative.replace(/\\/g, "/");
  const drive = /^[A-Za-z]:/.exec(current)?.[0] ?? "";

  let base: string;
  if (rel.startsWith("/") || /^[A-Za-z]:\//.test(rel)) {
    // Already absolute
    base = rel;
  } else {
    // Get directory of the current file
    const lastSlash = current.lastIndexOf("/");
    base = (lastSlash >= 0 ? current.substring(0, lastSlash) : "") + "/" + rel;
  }
  const relDrive = /^[A-Za-z]:/.exec(base)?.[0];
  const prefix = relDrive ?? drive;
  if (relDrive) base = base.slice(2);

  // Split relative path and resolve . and ..
  const parts = base.split("/");
  const resolved: string[] = [];

  for (const part of parts) {
    if (part === "" || part === ".") continue;
    if (part === "..") {
      resolved.pop();
    } else {
      resolved.push(part);
    }
  }

  return prefix + "/" + resolved.join("/");
}

/**
 * Turn an image URL from a document into one the webview can load.
 * Remote and data URLs pass through; local paths (relative to the document,
 * or absolute) are served by the backend's `mrimg` protocol.
 */
export function resolveImageSrc(currentPath: string | null, url: string): string {
  if (!url || /^[a-z][a-z0-9+.-]+:/i.test(url) || url.startsWith("#")) return url;
  if (!currentPath) return url;

  const pathPart = url.split(/[?#]/)[0];
  let decoded = pathPart;
  try {
    decoded = decodeURI(pathPart);
  } catch {
    // Malformed escapes: use the path as written
  }
  return convertFileSrc(resolvePath(currentPath, decoded), "mrimg");
}
