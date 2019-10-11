import { Component, OnInit, Input, EventEmitter } from '@angular/core';
import {FoosService} from '../../../services/foos.service';
import { ActivatedRoute, Router } from '@angular/router';
import { IModalComponent } from 'src/services/modal/modal.service';
import { formatDate } from '@angular/common';

@Component({
  selector: 'app-playerstats',
  templateUrl: './playerstats.component.html',
  styleUrls: ['./playerstats.component.scss']
})
export class PlayerStatsComponent implements OnInit, IModalComponent {
  public onDismiss = new EventEmitter();
  public params?: any;

  constructor(private route: ActivatedRoute, private foosService: FoosService, private router: Router) { }


  public playerId = 0;
  public stats;
  public statsKeys;
  public games;
  public loadingMessage = 'Loading player stats ...';

  ngOnInit() {
    if (!this.foosService.loggedIn) {
      this.router.navigateByUrl(`login`);
    }
    this.playerId = this.params.playerId;
    this.loadLeaderboard();
  }

  private loadLeaderboard() {
    this.foosService.getPlayersStats(this.playerId)
      .then((stats) => {
        this.stats = stats;
        this.statsKeys = Object.keys(this.stats).filter(x => x !== 'name');
      })
      .catch((err) => {
        this.loadingMessage = 'Couldn\'t find any stats for the player';
      });

    this.foosService.getPlayerGames(this.playerId)
      .then((games) => {
        this.games = games;
      })
      .catch((err) => {
        this.loadingMessage = 'Couldn\'t find and games for the player';
      });
  }

  formatDate(date: string): string {
    const jsDate = new Date(date);
    return jsDate.toLocaleDateString() + ' ' + jsDate.toLocaleTimeString();
  }

  close() {
    this.onDismiss.next();
  }
}
