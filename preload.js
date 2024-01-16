const { contextBridge, ipcRenderer } = require("electron");

/**
 * The preload script runs before. It has access to web APIs
 * as well as Electron's renderer process modules and some
 * polyfilled Node.js functions.
 *
 * https://www.electronjs.org/docs/latest/tutorial/sandbox
 */
window.addEventListener('DOMContentLoaded', () => {
  const replaceText = (selector, text) => {
    const element = document.getElementById(selector)
    if (element) element.innerText = text
  }

  for (const type of ['chrome', 'node', 'electron']) {
    replaceText(`${type}-version`, process.versions[type])
  }
});

// Exponer Renderer para que sea visible desde Angular
contextBridge.exposeInMainWorld('api', {
  onFileOpen: callback => ipcRenderer.on('file:open', (_event, value) => callback(value)),
  onFileConvert: callback => ipcRenderer.on('file:convert', (_event, value) => callback(value)),
});
