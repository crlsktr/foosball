import { Component, OnInit } from '@angular/core';
import {FoosService} from '../../../services/foos.service';

@Component({
  selector: 'app-teamleaderboard',
  templateUrl: './teamleaderboard.component.html',
  styleUrls: ['./teamleaderboard.component.scss']
})
export class TeamLeaderboardComponent implements OnInit {

  public leaders = [];
  public sortedLeaders = [];
  public showVideo = false;
  private sortColumn = 'position';
  private sortDesc = true;

  constructor(private foosService: FoosService) { }

  ngOnInit() {
    this.loadLeaderboard();
  }

  private loadLeaderboard() {
    this.foosService.loadTeamLeaderboard()
      .then((leaders) => {
        this.leaders = leaders;
        this.sortLeaders(this.sortColumn);
      });
  }

  sortLeaders(column: string) {
    if (this.sortColumn === column) {
      this.sortDesc = !this.sortDesc;
    } else {
      this.sortColumn = column;
      this.sortDesc = true;
    }

    this.sortedLeaders = this.leaders.sort((a, b) => {
      const columnType = typeof a[column];
      if (columnType === 'number') {
        return this.sortDesc ? b[column] - a[column] : a[column] - b[column];
      } else if (columnType === 'string') {
        return this.sortDesc ? a[column].localeCompare(b[column]) : b[column].localeCompare(a[column]);
      }
      return 0;
    });
  }
}
