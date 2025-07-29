import { Component, OnInit, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faDesktop, faHouse, faMoon, faSun } from '@fortawesome/free-solid-svg-icons';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, FontAwesomeModule],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App implements OnInit {
  protected readonly title = signal('portfolio-manager-frontent');

  faHouse = faHouse;

  themeIcon = faDesktop;
  faMoon = faMoon;
  faDesktop = faDesktop;
  faSun = faSun;

  isMenuOpened = false;
  isThemeMenuOpened = false;

  theme = 'system';

  // load the theme on page load
  ngOnInit(): void {
    const localTheme = localStorage.getItem('theme');
    if (localTheme !== null) {
      this.setTheme(localTheme);
    } else {
      this.updateDataTheme();
    }
  }

  // update the html tag to load the theme
  updateDataTheme(): void {
    if (this.theme === 'system') {
      document.documentElement.removeAttribute('data-theme');
    } else {
      document.documentElement.setAttribute('data-theme', this.theme);
    }
  }

  // set the theme, save it in local storage and load it
  setTheme(theme: string): void {
    this.theme = theme;
    switch (this.theme) {
      case 'light':
        this.themeIcon = this.faSun;
        break;
      case 'dark':
        this.themeIcon = this.faMoon;
        break;
      case 'system':
        this.themeIcon = this.faDesktop;
        break;
    }
    localStorage.setItem('theme', this.theme);
    this.updateDataTheme();
  }

  // handle the burger menu display
  toggleMenu(): void {
    this.isMenuOpened = !this.isMenuOpened;
  }

  // handle the theme menu display
  toggleThemeMenu(): void {
    this.isThemeMenuOpened = !this.isThemeMenuOpened;
  }
}
