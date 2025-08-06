import { ComponentFixture, TestBed } from '@angular/core/testing';
import { Header } from './header';
import { provideHttpClient, withInterceptorsFromDi } from '@angular/common/http';
import { provideRouter } from '@angular/router';
import { By } from '@angular/platform-browser';

describe('Header', () => {
  let component: Header;
  let fixture: ComponentFixture<Header>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [Header],
      providers: [
        provideHttpClient(withInterceptorsFromDi()),
        provideRouter([])
      ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(Header);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should call setTheme when theme is clicked and change theme', () => {
    const htmlEl = document.documentElement;

    spyOn(component, 'setTheme').and.callThrough();

    const elements = fixture.debugElement.queryAll(
      By.css('.navbar-end .navbar-dropdown .navbar-item')
    );

    const element = elements[1].nativeElement;
    element.click();
    fixture.detectChanges();

    expect(component.setTheme).toHaveBeenCalledWith('dark');

    const newTheme = htmlEl.getAttribute('data-theme');
    expect(newTheme).toBe('dark');
  });
});
