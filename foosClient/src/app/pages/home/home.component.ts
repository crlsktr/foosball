import { Component, OnInit } from '@angular/core';
import {FoosService} from '../../../services/foos.service';
import {Router} from '@angular/router';
import {GAME_TYPES} from '../../../static/foosTypes';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  public players = [];
  public selectedPlayer;

  constructor(private foosService: FoosService, private router: Router) { }

  ngOnInit() {
    this.foosService.getAllPlayers()
    .then((players) => {
      this.players = players;
    });
  }

  public newMatch(isGauntlet) {
    const type = isGauntlet ? GAME_TYPES.GAUNTLET : GAME_TYPES.MATCH;
    this.router.navigateByUrl(`new/match/${type}`);
  }

  public getPlayerStats() {
    this.router.navigateByUrl(`playerstats/${this.selectedPlayer.id}`);
  }
}
