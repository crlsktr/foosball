import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { IndividualLeaderboardComponent } from './individual/individual-leaderboard.component';
import { TeamLeaderboardComponent } from './team/team-leaderboard.component';
import { CommonModule } from '@angular/common';
import { PipesModule } from 'src/app/pipes/pipes.module';
import { RouterModule } from '@angular/router';

@NgModule({
    imports: [
        BrowserModule,
        CommonModule,
        PipesModule,
        RouterModule,
    ],
    exports: [
      IndividualLeaderboardComponent,
      TeamLeaderboardComponent
    ],
    declarations: [
      IndividualLeaderboardComponent,
      TeamLeaderboardComponent,
    ],
    providers: [],
})
export class LeaderBoardsModule { }
