import { Component, OnInit } from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import * as _ from 'lodash';
import {FoosService} from '../../../services/foos.service';

@Component({
  selector: 'app-new-match',
  templateUrl: './new-match.component.html',
  styleUrls: ['./new-match.component.scss']
})
export class NewMatchComponent implements OnInit {

  public allPlayers = [];
  public activePlayers = [];

  public numOfPlayers;
  constructor(private route: ActivatedRoute, private foosService: FoosService) { }

  ngOnInit() {
    this.foosService.getAllUsers()
      .then((users) => {
        this.allPlayers = users;
      });

    this.route.params
      .subscribe((params) => {
        if (params && params.gameType) {
          this.numOfPlayers = +params.gameType;
          this.activePlayers = _.fill(Array(this.numOfPlayers), null);
        }
      });
  }

  public startGame() {
    this.activePlayers;
    debugger;

  }
}
