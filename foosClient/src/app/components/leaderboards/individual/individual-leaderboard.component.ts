import {
  EventEmitter,
  Component,
  OnInit,
  Output,
  Input,
  OnChanges,
  SimpleChanges,
  SimpleChange,
  OnDestroy
} from '@angular/core';
import { FoosService } from 'src/services/foos.service';
import {BehaviorSubject} from 'rxjs';

interface Leader {
  name: string;
}

@Component({
    selector: 'app-individual-leaderboard',
    templateUrl: 'individual-leaderboard.component.html',
    styleUrls: ['individual-leaderboard.component.scss'],
})
export class IndividualLeaderboardComponent implements OnInit, OnChanges, OnDestroy {
  @Input() reloadStats = new BehaviorSubject(false);
  @Input() search: string;

  public searchText = '';
  public leaders = new Array<Leader>();
  public filteredLeaders = new Array<Leader>();
  public sortedLeaders = new Array<Leader>();
  public showVideo = false;
  private sortColumn = 'ranking';
  private sortDesc = true;
  private sub;
  constructor(private foosService: FoosService) { }

  ngOnInit() {
    this.loadLeaderboard();
    this.searchText = this.search;
    this.sub = this.reloadStats.subscribe((shouldReload) => {
      if (shouldReload) {
        this.loadLeaderboard();
      }
    });
  }
  ngOnDestroy(): void {
    this.sub.unsubscribe();
  }

  private loadLeaderboard() {
    this.foosService.loadLeaderboard()
      .then((leaders) => {
        this.leaders = leaders;
        this.filterLeaders();
        this.sortLeaders(this.sortColumn, true);
      });
  }

  ngOnChanges(changes: SimpleChanges) {
    const search: SimpleChange = changes.search;
    this.searchText = search.currentValue.toLowerCase();
    this.filterLeaders();
    this.sortLeaders('ranking', true);
  }

  filterLeaders() {
    if (this.searchText) {
      this.filteredLeaders = this.leaders.filter((leader) => leader.name.toLowerCase().includes(this.searchText));
    } else {
      this.filteredLeaders = this.leaders;
    }
  }

  sortLeaders(column: string, keepdirection: boolean = false) {
    if (!keepdirection) {
      if (this.sortColumn === column ) {
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
