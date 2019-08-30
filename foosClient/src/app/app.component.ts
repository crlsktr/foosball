import { Component } from '@angular/core';
import {FoosService} from '../services/foos.service';
import {ModalService} from '../services/modal/modal.service';
import {NewMatchComponent} from './pages/new-match/new-match.component';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  constructor( private foosService: FoosService, private modalService: ModalService) {
    // this.foosService.login();
    this.foosService.isUserAuthenticated();
    // Example of how to use modal service
    // this.modalService.create(NewMatchComponent, {params: {test: 'hi'}})
    //   .then((data) => {
    //     console.log('After modal close');
    //   });

  }
}
