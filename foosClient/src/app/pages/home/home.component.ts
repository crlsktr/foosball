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
  public users = [];
  public selectedPlayer;
  public newPlayerName = '';
  public newPlayerUser;
  public teamPlayerOne;
  public teamPlayerTwo;

  constructor(private foosService: FoosService, private router: Router) { }

  ngOnInit() {
    if (!this.foosService.loggedIn) {
      this.router.navigateByUrl(`login`);
    }
    this.loadPlayersAndUsers();
  }

  public loadPlayersAndUsers() {
    this.foosService.getAllPlayers()
    .then((players) => {
      this.players = players;
    });

    this.foosService.getAllUsers()
    .then((users) => {
      this.users = users;
      this.users.push({username: 'no user', id: 0});
    });
  }

  public newMatch(isGauntlet) {
    const type = isGauntlet ? GAME_TYPES.GAUNTLET : GAME_TYPES.MATCH;
    this.router.navigateByUrl(`new/match/${type}`);
  }

  public getPlayerStats() {
    this.router.navigateByUrl(`playerstats/${this.selectedPlayer.id}`);
  }

  public createPlayer() {
    if (!this.newPlayerUser || !this.newPlayerUser.id || this.newPlayerUser.id === 0) {
      this.foosService.createPlayer(this.newPlayerName, null)
        .then(() => {
          this.newPlayerUser = null;
          this.newPlayerName = '';
          this.loadPlayersAndUsers();
        });
    } else {
      this.foosService.createPlayer(this.newPlayerName, this.newPlayerUser.id)
        .then(() => {
          this.newPlayerUser = null;
          this.newPlayerName = '';
          this.loadPlayersAndUsers();
        });
    }
  }

  public getTeamStats() {
    this.router.navigateByUrl(`teamstats/${this.teamPlayerOne.id}/${this.teamPlayerTwo.id}`);
  }
}
