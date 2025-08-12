import { Component, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faEnvelope, faLock } from '@fortawesome/free-solid-svg-icons';
import { AuthService } from '../../core/auth/auth-service';
import { ActivatedRoute, Router } from '@angular/router';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'app-login',
  imports: [FontAwesomeModule, FormsModule],
  templateUrl: './login.html',
  styleUrl: './login.scss'
})
export class Login implements OnInit {
  faEnvelope = faEnvelope;
  faLock = faLock;

  email = '';
  password = '';
  errorMessage = '';

  constructor(private titleService: Title, private authService: AuthService, private router: Router, private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.titleService.setTitle('Log in - Portfolio Manager');

    if (this.authService.isAuthenticated()) {
      this.router.navigate(['']);
      return;
    }
  }

  onSubmit(): void {
    this.authService.login({ email: this.email, password: this.password }).subscribe({
      next: () => {
        this.router.navigate(['']);
      },
      error: () => {
        this.errorMessage = 'Invalid credentials.';
      }
    });
  }
}
