import { Component, OnInit } from '@angular/core';
import {FoosService} from '../../../services/foos.service';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-playerstats',
  templateUrl: './playerstats.component.html',
  styleUrls: ['./playerstats.component.scss']
})
export class PlayerStatsComponent implements OnInit {

  constructor(private route: ActivatedRoute, private foosService: FoosService) { }

  public playerId = 0;
  public stats;
  public statsKeys;

  ngOnInit() {
    this.route.params.subscribe( params => {
      this.playerId = +params['playerId'];
      this.loadLeaderboard();
    });
  }

  private loadLeaderboard() {
    this.foosService.getPlayersStats(this.playerId)
      .then((stats) => {
        this.stats = stats;
        this.stats.percentage = this.stats.percentage * 100;
        this.statsKeys = Object.keys(this.stats).filter(x => x !== 'name');
      });
  }
}
