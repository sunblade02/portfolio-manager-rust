import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, tap } from 'rxjs';
import { jwtDecode } from 'jwt-decode';
import { environment } from '../../../environments/environments';

interface JwtPayload {
  exp: number; // timestamp UNIX
  sub: string;
  // ajoute ici d’autres champs si ton JWT en contient
}

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private http = inject(HttpClient);

  private loginUrl = environment.apiBaseUrl + '/api/auth/login';

  login(credentials: { email: string, password: string }): Observable<string> {
    return this.http.post<string>(this.loginUrl, credentials).pipe(
      tap(jwt => {
        sessionStorage.setItem('jwt', jwt);
      })
    );
  }

  logout(): void {
    sessionStorage.removeItem('jwt');
  }

  isTokenExpired(): boolean {
    const decoded = this.decodeToken();
    if (!decoded?.exp) return true;

    const now = Math.floor(Date.now() / 1000); // en secondes
    return decoded.exp < now;
  }

  isAuthenticated(): boolean {
    return !!sessionStorage.getItem('jwt') && !this.isTokenExpired();
  }

  getToken(): string | null {
    return sessionStorage.getItem('jwt');
  }

  decodeToken(): JwtPayload | null {
    const token = this.getToken();
    if (!token) return null;

    try {
      return jwtDecode<JwtPayload>(token);
    } catch (e) {
      console.error('Erreur de décodage du token :', e);
      return null;
    }
  }
}