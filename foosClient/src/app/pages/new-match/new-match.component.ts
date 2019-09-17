import {Component, EventEmitter, OnInit} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import * as _ from 'lodash';
import {FoosService} from '../../../services/foos.service';
import {IModalComponent} from '../../../services/modal/modal.service';

export interface GameResult {
  id: number;
  winners: number;
  spread: number;
}

@Component({
  selector: 'app-new-match',
  templateUrl: './new-match.component.html',
  styleUrls: ['./new-match.component.scss']
})
export class NewMatchComponent implements OnInit, IModalComponent {

  public onDismiss = new EventEmitter();
  public params: any = {};

  public allPlayers = [];
  public activePlayers = [];
  public numbers = [];
  public numOfPlayers;
  public games = [];
  public gameResults: GameResult[] = [];
  public errMsg = '';


  constructor(private route: ActivatedRoute, private foosService: FoosService, private router: Router) { }

  ngOnInit() {

    this.foosService.getAllPlayers()
      .then((players) => {
        this.allPlayers = players;
      });

    this.setGameMode(4);
  }

  public setGameMode(numPlayers) {
    this.numOfPlayers = +numPlayers;
    this.numbers = _.fill(Array(this.numOfPlayers), null);
    this.activePlayers = _.fill(Array(this.numOfPlayers), null);
  }

  public startGame() {
    this.foosService.startGame(this.activePlayers)
      .then((data) => {
        if (data && data.games) {
          this.games = data.games;
          _.forEach(data.games, (game) => {
            this.gameResults.push({
              id: game.id,
              winners: null,
              spread: null,
            });
          });
        }
      });
  }

  public finishGame() {
    this.errMsg = '';
    _.forEach(this.gameResults, (results) => {
      results.spread = +results.spread;
      if (!results.winners) {
        this.errMsg = 'You\'re missing data dawg..';
      }
    });

    if (!this.errMsg) {
      this.foosService.finishGame(this.gameResults)
        .then(() => {
          this.onDismiss.next();
        });
    }
  }

  close() {
    this.onDismiss.next();
  }
}
