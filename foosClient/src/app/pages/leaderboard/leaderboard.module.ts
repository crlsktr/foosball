import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LeaderboardComponent } from './leaderboard.component';
import {PipesModule} from '../../pipes/pipes.module';

@NgModule({
  declarations: [LeaderboardComponent],
  imports: [
    CommonModule,
    PipesModule,
  ]
})
export class LeaderboardModule { }
