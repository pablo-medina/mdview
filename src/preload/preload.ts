// See the Electron documentation for details on how to use preload scripts:
// https://www.electronjs.org/docs/latest/tutorial/process-model#preload-scripts
import { contextBridge, ipcRenderer } from "electron";

window.addEventListener('DOMContentLoaded', () => {
    const replaceText = (selector: string, text: string) => {
        const element = document.getElementById(selector)
        if (element) element.innerText = text
    }

    for (const type of ['chrome', 'node', 'electron']) {
        replaceText(`${type}-version`, process.versions[type])
    }
});

// Exponer Renderer para que sea visible desde Angular
contextBridge.exposeInMainWorld('api', {
    onFileOpen: (callback: (arg0: any) => void) => ipcRenderer.on('file:open', (_event, value) => callback(value)),
    onFileConvert: (callback: (arg0: any) => void) => ipcRenderer.on('file:convert', (_event, value) => callback(value)),
});
