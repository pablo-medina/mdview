import { app, BrowserWindow, dialog, Menu, PrintToPDFOptions } from 'electron';
import path from 'path';
import fs from 'fs';
import * as marked from 'marked';

let mainWindow: Electron.BrowserWindow = undefined;
let currentFile: string = undefined;

async function convertMarkdownToHTML(mdFilePath: string): Promise<string> {
  console.log('Archivo:', mdFilePath);
  const mdBuffer = fs.readFileSync(mdFilePath);
  const mdContent = mdBuffer.toString();
  const htmlContent = marked.parse(mdContent);
  console.log('HTML Generado:', htmlContent);
  console.log('WEBC:', mainWindow.webContents);

  const content = `<div class="markdown-content">${htmlContent}</div>`;
  return content;
}

async function convertHTMLToPDF(htmlString: string, pdfFilePath: string): Promise<boolean> {
  const options: PrintToPDFOptions = {
    pageSize: 'A4',
    printBackground: true,
    landscape: false
  }
  return mainWindow.webContents.printToPDF(options).then(data => {
    fs.writeFileSync(pdfFilePath, data);
    console.log(`PDF generado en: ${pdfFilePath}`);
    return true;
  });

}

const openFile = () => {
  // Mostrar cuadro de diálogo para abrir archivo
  dialog.showOpenDialog(mainWindow, {
    properties: ['createDirectory'],
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
      convertMarkdownToHTML(filePath)
        .then(htmlContent => {
          console.log('HTML Generado:', htmlContent);
          mainWindow.webContents.send('file:open', { path: filePath, content: htmlContent });
          currentFile = filePath;
          Menu.getApplicationMenu().getMenuItemById('convert').enabled = true;
        }).catch(err => {
          console.error(err);
        })
    }
  }).catch(err => {
    console.error(err);
  })
}

const convertFile = () => {
  // Mostrar cuadro de diálogo para guardar archivo
  if (currentFile) {
    dialog.showSaveDialog(mainWindow, {
      properties: ['showOverwriteConfirmation'],
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

        // Convertir el archivo Markdown a HTML
        return convertMarkdownToHTML(currentFile)
          .then(htmlString => convertHTMLToPDF(htmlString, outputFilePath))
          .then(() => {
            console.log(`Se ha creado el archivo PDF en: ${outputFilePath}`);
            mainWindow.webContents.send('file:convert', outputFilePath);
          })
          .catch(err => {
            console.error(err);
          })
      }
    }).catch(err => {
      console.error(err);
    })
  }
}

const buildDefaultMenu = (): Electron.Menu => {
  return Menu.buildFromTemplate([
    {
      label: 'Archivo',
      submenu: [
        { label: 'Abrir', click: openFile },
        { type: 'separator' },
        { id: 'convert', label: 'Exportar a PDF...', click: convertFile, enabled: false },
        { type: 'separator' },
        { role: 'quit', label: 'Salir' } // Role quit implementa cierre automático
      ]
    },
    {
      label: 'Desarrollo',
      submenu: [
        { role: 'toggleDevTools', label: 'Mostrar/Ocultar DevTools' }
      ]
    }
  ]);
}

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (require('electron-squirrel-startup')) {
  app.quit();
}

const createWindow = (): BrowserWindow => {
  // Create the browser window.
  mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    title: 'MDView',
    //backgroundMaterial: 'acrylic',
    resizable: true,
    maximizable: true,
    useContentSize: false,
    autoHideMenuBar: false,
    webPreferences: {
      preload: path.join(__dirname, './preload.js'),
      contextIsolation: true
    },
    show: false
  });

  // Cargar index.html de Angular
  mainWindow.loadFile(path.join(__dirname, `browser/index.html`));

  // Agregar el menú personalizado - Si uso mainWindow.setMenu() cuango hago getApplicationMenu me trae el menú por defecto de Electron
  Menu.setApplicationMenu(buildDefaultMenu());

  // Open the DevTools.
  //mainWindow.webContents.openDevTools();

  mainWindow.show();

  return mainWindow;
};

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on('ready', () => {
  const mw = createWindow();
  mainWindow = mw;
});

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  // On OS X it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and import them here.
