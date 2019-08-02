import { Component } from '@angular/core';
import {FoosService} from '../services/foos.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  constructor( private foosService: FoosService) {
    // this.foosService.login();
    this.foosService.isUserAuthenticated();
  }
}
