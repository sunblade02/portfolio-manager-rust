import { Component, OnInit } from '@angular/core';
import { Router, RouterLink } from '@angular/router';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faDesktop, faHouse, faMoon, faSun } from '@fortawesome/free-solid-svg-icons';
import { AuthService } from '../../../core/auth/auth-service';

@Component({
  selector: 'app-header',
  imports: [FontAwesomeModule, RouterLink],
  templateUrl: './header.html',
  styleUrl: './header.scss'
})
export class Header implements OnInit {
  faHouse = faHouse;

  themeIcon = faDesktop;
  faMoon = faMoon;
  faDesktop = faDesktop;
  faSun = faSun;

  isMenuOpened = false;
  isThemeMenuOpened = false;

  theme = 'system';

  constructor(public authService: AuthService, private router: Router) {}

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

  logout() {
    this.authService.logout();
    this.router.navigate(['']);
  }
}
