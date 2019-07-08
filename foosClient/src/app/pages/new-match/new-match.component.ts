import { Component, OnInit } from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import * as _ from 'lodash';

@Component({
  selector: 'app-new-match',
  templateUrl: './new-match.component.html',
  styleUrls: ['./new-match.component.scss']
})
export class NewMatchComponent implements OnInit {

  public allPlayers = [];
  public activePlayers = [];

  public numOfPlayers;
  constructor(private route: ActivatedRoute) { }

  ngOnInit() {
    this.route.params
      .subscribe((params) => {
        if (params && params.gameType) {
          this.numOfPlayers = +params.gameType;
          this.activePlayers = _.fill(Array(this.numOfPlayers), null);
        }
      });
  }

}
