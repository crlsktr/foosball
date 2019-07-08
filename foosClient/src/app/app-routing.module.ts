import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import {HomeComponent} from './pages/home/home.component';
import {HomeModule} from './pages/home/home.module';
import {LeaderboardComponent} from './pages/leaderboard/leaderboard.component';
import {LeaderboardModule} from './pages/leaderboard/leaderboard.module';
import {NewMatchComponent} from './pages/new-match/new-match.component';
import {NewMatchModule} from './pages/new-match/new-match.module';

const routes: Routes = [
  {path: '', pathMatch: 'full', redirectTo: 'home'},
  {path: 'home', component: HomeComponent, },
  {path: 'leaderboard', component: LeaderboardComponent},
  {path: 'new/match/:gameType', component: NewMatchComponent},
];

@NgModule({
  imports: [
    HomeModule,
    LeaderboardModule,
    NewMatchModule,
    RouterModule.forRoot(routes)
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }
