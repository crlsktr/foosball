import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { FoosService } from 'src/services/foos.service';
import {HistoryPoint} from '../../components/time-series/history-point';



interface TeamStat {
	opposing_player_one: string,
	opposing_player_two: string,
	opposing_team_id: number,
	won: boolean,
	points: number,
	opponent_points: number,
	played_on: Date,
	change: number,
	current_ranking: number,
	highlight: boolean,
}

@Component({
	selector: 'app-team-stats',
	templateUrl: './teamstats.component.html',
	styleUrls: ['./teamstats.component.scss']
})
export class TeamStatsComponent implements OnInit {

	constructor(private route: ActivatedRoute, private foosService: FoosService, private router: Router) { }
  public mappedhistory: HistoryPoint[];
	public details: any;
	public teamId = 0;
	public stats: TeamStat[];
	public statsKeys;
	public loadingMessage = 'Loading player stats ...';

	public isGraphVisible: boolean = false;



	ngOnInit() {
		if (!this.foosService.loggedIn) {
			this.router.navigateByUrl(`login`);
		}

		this.route.params.subscribe(params => {
			this.teamId = +params['teamId'];
			this.loadTeamDetail();
			this.loadStats();
		});
	}

	loadTeamDetail() {
		this.foosService.getTeamDetail(this.teamId)
			.then((detail) => {
				this.details = detail;
			})
	}

	private loadStats() {
		this.foosService.getTeamGames(this.teamId)
			.then((stats: TeamStat[]) => {
				this.stats = stats;
				this.mappedhistory = this.stats.map<HistoryPoint>(x => ({ played: new Date(x.played_on), ranking: x.current_ranking }));
			})
			.catch((err) => {
				this.loadingMessage = `Couldn't find any stats for the team, ${err}`;
			});
	}
}
