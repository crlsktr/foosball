// src/app/auth/auth-guard.service.tsimport { Injectable } from '@angular/core';
import { Router, CanActivate } from '@angular/router';
import { Injectable } from '@angular/core';
import { FoosService } from './foos.service';

@Injectable()
export class AuthGuardService implements CanActivate {

  constructor(public foos: FoosService, public router: Router) {}

  async canActivate(): Promise<boolean> {
    // this ones checks if we already have a valid cookie
    // and set the logged in status
    if (!this.foos.loggedIn) {
      await this.foos.isUserAuthenticated();
    }

    // this one redirects us if the first one failed.
    if (!this.foos.loggedIn) {
      this.router.navigate(['login']);
      return false;
    }
    return true;
  }}
