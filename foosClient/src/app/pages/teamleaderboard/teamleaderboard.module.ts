import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TeamLeaderboardComponent } from './teamleaderboard.component';
import {PipesModule} from '../../pipes/pipes.module';

@NgModule({
  declarations: [TeamLeaderboardComponent],
  imports: [
    CommonModule,
    PipesModule,
  ]
})
export class TeamLeaderboardModule { }
