import { Component, OnInit } from '@angular/core';
import {FoosService} from '../../../services/foos.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  public players = [];

  constructor(private foosService: FoosService) { }

  ngOnInit() {
  }

  public testRoute() {
    this.foosService.searchUser2();
  }

}
