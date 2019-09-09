import { Component, OnInit } from '@angular/core';
import {FoosService} from '../../../services/foos.service';
import {Router} from '@angular/router';
import {GAME_TYPES} from '../../../static/foosTypes';
import { ModalService } from 'src/services/modal/modal.service';
import { NewMatchComponent } from '../new-match/new-match.component';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  public statSelection = 'individual';
  public searchText = '';

  constructor(private foosService: FoosService, private router: Router, private modalService: ModalService) { }

  ngOnInit() {
  }

  public setStatSelection(selection) {
    this.statSelection = selection;
  }

  public startGame() {
    this.modalService.create(NewMatchComponent, {params: {test: 'hi'}})
      .then((data) => {
        console.log('After modal close');
      });
  }

}
