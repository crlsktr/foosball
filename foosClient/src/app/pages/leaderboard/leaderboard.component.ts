import { Component, OnInit } from '@angular/core';
import {FoosService} from '../../../services/foos.service';

@Component({
  selector: 'app-leaderboard',
  templateUrl: './leaderboard.component.html',
  styleUrls: ['./leaderboard.component.scss']
})
export class LeaderboardComponent implements OnInit {

  public leaders = [];
  public showVideo = false;

  constructor(private foosService: FoosService) { }

  ngOnInit() {
    this.loadLeaderboard();
  }

  private loadLeaderboard () {
    this.foosService.loadLeaderboard()
      .then((leaders) => {
        this.leaders = leaders;
      });
  }

}
