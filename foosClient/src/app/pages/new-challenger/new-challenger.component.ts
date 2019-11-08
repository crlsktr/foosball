import {Component, EventEmitter, OnInit} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import * as _ from 'lodash';
import {FoosService} from '../../../services/foos.service';
import {IModalComponent} from '../../../services/modal/modal.service';


@Component({
  selector: 'app-new-challenger',
  templateUrl: './new-challenger.component.html',
  styleUrls: ['./new-challenger.component.scss']
})
export class NewChallengerComponent implements OnInit, IModalComponent {

  public onDismiss = new EventEmitter();
  public params: any = {};

  public playerName = '';
  public playerUserId: number;
  public users = [];


  constructor(private route: ActivatedRoute, private foosService: FoosService, private router: Router) { }

  ngOnInit() {

    this.foosService.getAllUsers()
      .then((users) => {
        this.users = users;
      });
  }

  async addChallenger() {
    if (this.playerName !== '') {
      if (!this.playerUserId || this.playerUserId === 0) {
        await this.foosService.createPlayer(this.playerName, null);

      } else {
        await this.foosService.createPlayer(this.playerName, this.playerUserId);
      }
      this.onDismiss.next();
    } else {

    }
  }

  selectPlayer(event){
    this.playerUserId = event ? event.id : 0;
  }

  close() {
    this.onDismiss.next();
  }
}
