/** Resolve a relative path against the directory of the current document. */
export function resolvePath(currentPath: string, relative: string): string {
  // Get directory of the current file
  const lastSlash = currentPath.lastIndexOf("/");
  const dir = lastSlash >= 0 ? currentPath.substring(0, lastSlash) : "";

  // Split relative path and resolve . and ..
  const parts = (dir + "/" + relative).split("/");
  const resolved: string[] = [];

  for (const part of parts) {
    if (part === "" || part === ".") continue;
    if (part === "..") {
      resolved.pop();
    } else {
      resolved.push(part);
    }
  }

  return "/" + resolved.join("/");
}
