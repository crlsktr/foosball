// src/app/auth/auth-guard.service.tsimport { Injectable } from '@angular/core';
import { Router, CanActivate } from '@angular/router';
import { Injectable } from '@angular/core';
import { FoosService } from './foos.service';

@Injectable()
export class AuthGuardService implements CanActivate {

  constructor(public foos: FoosService, public router: Router) {}

  canActivate(): boolean {
    if (!this.foos.loggedIn) {
      this.router.navigate(['login']);
      return false;
    }
    return true;
  }}
