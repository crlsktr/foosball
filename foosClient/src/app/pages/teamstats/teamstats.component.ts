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

  public teamId = 0;
  public stats;
  public statsKeys;
  public loadingMessage = 'Loading player stats ...';

  ngOnInit() {
    if (!this.foosService.loggedIn) {
      this.router.navigateByUrl(`login`);
    }

    this.route.params.subscribe( params => {
      this.teamId = +params['teamId'];
      this.loadStats();
    });
  }

  private loadStats() {

      this.foosService.getTeamGames(this.teamId)
      .then((stats) => {
        console.log("loaded all stats", stats);
        this.stats = stats;
      })
      .catch((err) => {
        this.loadingMessage = 'Couldn\'t find any stats for the team';
      });

  }
}
