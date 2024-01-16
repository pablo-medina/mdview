import { ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';

interface FileOpenData {
  path: string
  content: string
}

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent implements OnInit {
  title = 'mdview';
  path = '';
  content = '<p>No se ha abierto ningún archivo</p>';

  constructor(private cdr: ChangeDetectorRef) {}

  ngOnInit(): void {
    const api = (<any>window).api;
    console.log('API:', api);
    api.onFileOpen((data: FileOpenData) => {
      console.debug('Received Data:', data);
      this.path = data.path;
      this.content = data.content;
      this.cdr.detectChanges();
    });
  }
}
