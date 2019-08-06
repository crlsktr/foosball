import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { FoosService } from 'src/services/foos.service';

@Component({
  selector: 'app-team-stats',
  templateUrl: './teamstats.component.html',
  styleUrls: ['./teamstats.component.scss']
})
export class TeamStatsComponent implements OnInit {

  constructor(private route: ActivatedRoute, private foosService: FoosService, private router: Router) { }

  public playerOneId = 0;
  public playerTwoId = 0;
  public stats;
  public statsKeys;
  public loadingMessage = 'Loading player stats ...';

  ngOnInit() {
    if (!this.foosService.loggedIn) {
      this.router.navigateByUrl(`login`);
    }

    this.route.params.subscribe( params => {
      this.playerOneId = +params['playerOneId'];
      this.playerTwoId = +params['playerTwoId'];
      this.loadStats();
    });
  }

  private loadStats() {
    this.foosService.getTeamStats(this.playerOneId, this.playerTwoId)
      .then((stats) => {
        this.stats = stats;
        this.stats.percentage = this.stats.percentage * 100;
        this.statsKeys = Object.keys(this.stats).filter(x => x !== 'name');
      })
      .catch((err) => {
        this.loadingMessage = 'Couldn\'t find any stats for the team';
      });
  }
}
