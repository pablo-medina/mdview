import { app, BrowserWindow, dialog, Menu } from 'electron';
import { fileURLToPath } from 'url';
import { dirname } from 'path';
import fs from 'fs';
import * as marked from 'marked';
import markdownpdf from 'markdown-pdf';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

console.log('DIRNAME:', __dirname);

let mainWindow;

let currentFile = undefined;

const openFile = () => {
    // Mostrar cuadro de diálogo para abrir archivo
    dialog.showOpenDialog(mainWindow, {
        properties: ['openFile'],
        filters: [
            {
                name: 'Archivos Markdown (*.md)',
                extensions: ['md']
            }, {
                name: 'Todos los archivos (*.*)',
                extensions: ['*']
            }
        ]
    }).then(result => {
        if (!result.canceled) {
            const filePath = result.filePaths[0];
            console.log('Archivo seleccionado:', filePath);
            const mdBuffer = fs.readFileSync(filePath);
            const mdContent = mdBuffer.toString();
            const htmlContent = marked.parse(mdContent);
            console.log('HTML Generado:', htmlContent);
            mainWindow.webContents.send('file:open', { path: filePath, content: `<div class="markdown-content">${htmlContent}</div>` });
            currentFile = filePath;
        }
    }).catch(err => {
        console.error(err);
    }) 
}

const convertFile = () => {
    // Mostrar cuadro de diálogo para guardar archivo
    if (currentFile) {
        dialog.showSaveDialog(mainWindow, {
            properties: ['saveFile'],
            filters: [
                {
                    name: 'Archivos PDF (*.pdf)',
                    extensions: ['pdf']
                }
            ]
        }).then(result => {
            if (!result.canceled) {
                const outputFilePath = result.filePath;
                console.log('Archivo a guardar:', outputFilePath);

                // Convertir el archivo markdown a PDF
                markdownpdf({cssPath: './estilos.css'}).from.path(currentFile).to(outputFilePath, () => {
                    console.log(`Se ha creado el archivo PDF en: ${outputFilePath}`);
                    mainWindow.webContents.send('file:convert', outputFilePath);
                })
            }
        }).catch(err => {
            console.error(err);
        })
    }
}

const toggleDevTools = () => {
    if (mainWindow.webContents.isDevToolsOpened()) {
        mainWindow.webContents.closeDevTools();
    } else {
        mainWindow.webContents.openDevTools();
    }
}

const defaultMenu = Menu.buildFromTemplate([
    {
        label: 'Archivo',
        submenu: [
            { role: 'open', label: 'Abrir', click: openFile },
            { role: 'pdf', label: 'Convertir a PDF...', click: convertFile },
            { type: 'separator' },
            { role: 'quit', label: 'Salir' } // Role quit implementa cierre automático
        ]
    },
    {
        label: 'Ver',
        submenu: [
            { role: 'reload', label: 'Recargar' },
            { role: 'toggle-devtools', label: 'Habilitar DevTools', click: toggleDevTools },
            { type: 'separator' },
            { role: 'resetzoom', label: 'Reiniciar Zoom' },
            { role: 'zoomin', label: 'Acercar' },
            { role: 'zoomout', label: 'Alejar' }
        ]
    }
]);

const createWindow = () => {
    mainWindow = new BrowserWindow({
        width: 800,
        height: 600,
        title: 'Angular Electron',
        resizable: true,
        webPreferences: {
            preload: `${app.getAppPath()}/preload.js`,
            contextIsolation: true
        }
    });

    const indexFile = `file://${__dirname}/dist/browser/index.html`;
    console.log('INDEX:', indexFile);

    mainWindow.loadURL(indexFile);
    mainWindow.setMenu(defaultMenu);
}

app.whenReady().then(() => {
    createWindow();

    app.on('activate', () => {
        // On macOS it's common to re-create a window in the app when the
        // dock icon is clicked and there are no other windows open.
        if (BrowserWindow.getAllWindows().length === 0) createWindow();
    });
});

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', function () {
    if (process.platform !== 'darwin') app.quit()
});

