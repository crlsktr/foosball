import { Component, OnInit } from '@angular/core';
import {FoosService} from '../../../services/foos.service';
import {Router} from '@angular/router';
import {GAME_TYPES} from '../../../static/foosTypes';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss']
})
export class LoginComponent implements OnInit {

  public loginUsername = '';
  public loginPassword = '';
  public createUsername = '';
  public createPassword = '';
  public loggingIn = false;
  public createUserContext = false;

  constructor(private foosService: FoosService, private router: Router) { }

  ngOnInit() {
  }

  async attemptLogin() {
    this.loggingIn = true;
    const user = await this.foosService.login(this.loginUsername, this.loginPassword);
    this.loggingIn = false;
    this.router.navigateByUrl(`home`);
    console.log(user);
  }

  async createUser() {
    const user = await this.foosService.createUser(this.createUsername, this.createPassword);
    console.log(user);
  }
}
