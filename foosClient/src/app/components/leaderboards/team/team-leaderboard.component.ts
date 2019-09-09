import { EventEmitter, Component, OnInit, Output, Input, SimpleChanges, OnChanges, SimpleChange } from '@angular/core';
import { FoosService } from 'src/services/foos.service';

interface TeamLeader {
  player_one_name: string;
  player_two_name: string;
}

@Component({
    selector: 'app-team-leaderboard',
    templateUrl: 'team-leaderboard.component.html',
    styleUrls: ['team-leaderboard.component.scss'],
})
export class TeamLeaderboardComponent implements OnInit, OnChanges {
  @Input() search: string;

  public searchText = '';
  public leaders = new Array<TeamLeader>();
  public filteredLeaders = new Array<TeamLeader>();
  public sortedLeaders = new Array<TeamLeader>();
  public showVideo = false;
  private sortColumn = 'position';
  private sortDesc = true;

  constructor(private foosService: FoosService) { }

  ngOnInit() {
    this.loadLeaderboard();
    this.searchText = this.search;
  }

  ngOnChanges(changes: SimpleChanges) {
    const search: SimpleChange = changes.search;
    this.searchText = search.currentValue.toLowerCase();
    this.filterLeaders();
    this.sortLeaders('ranking', true);
  }

  private loadLeaderboard() {
    this.foosService.loadTeamLeaderboard()
      .then((leaders) => {
        this.leaders = leaders;
        this.filterLeaders();
        this.sortLeaders(this.sortColumn);
      });
  }

  filterLeaders() {
    if (this.searchText) {
      this.filteredLeaders = this.leaders.filter((leader) => (
        leader.player_one_name.toLowerCase().includes(this.searchText)
        || leader.player_two_name.toLowerCase().includes(this.searchText)));
    } else {
      this.filteredLeaders = this.leaders;
    }
  }

  sortLeaders(column: string, keepdirection: boolean = false) {
    if (!keepdirection) {
      if (this.sortColumn === column) {
        this.sortDesc = !this.sortDesc;
      } else {
        this.sortColumn = column;
        this.sortDesc = true;
      }
    }

    this.sortedLeaders = this.filteredLeaders.sort((a, b) => {
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
