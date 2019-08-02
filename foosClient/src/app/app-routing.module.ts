import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import {HomeComponent} from './pages/home/home.component';
import {HomeModule} from './pages/home/home.module';
import {LeaderboardComponent} from './pages/leaderboard/leaderboard.component';
import {LeaderboardModule} from './pages/leaderboard/leaderboard.module';
import {NewMatchComponent} from './pages/new-match/new-match.component';
import {NewMatchModule} from './pages/new-match/new-match.module';
import { PlayerStatsComponent } from './pages/playerstats/playerstats.component';
import { PlayerStatsModule } from './pages/playerstats/playerstats.module';
import { LoginComponent } from './pages/login/login.component';
import { LoginModule } from './pages/login/login.module';

const routes: Routes = [
  {path: '', pathMatch: 'full', redirectTo: 'home'},
  {path: 'home', component: HomeComponent, },
  {path: 'leaderboard', component: LeaderboardComponent},
  {path: 'playerstats/:playerId', component: PlayerStatsComponent},
  {path: 'new/match/:gameType', component: NewMatchComponent},
  {path: 'login', component: LoginComponent}
];

@NgModule({
  imports: [
    HomeModule,
    LeaderboardModule,
    PlayerStatsModule,
    NewMatchModule,
    LoginModule,
    RouterModule.forRoot(routes, {useHash: true})
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }