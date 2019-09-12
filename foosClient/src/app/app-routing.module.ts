import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { HomeComponent } from './pages/home/home.component';
import { HomeModule } from './pages/home/home.module';
import { NewMatchComponent } from './pages/new-match/new-match.component';
import { NewMatchModule } from './pages/new-match/new-match.module';
import { PlayerStatsComponent } from './pages/playerstats/playerstats.component';
import { TeamStatsComponent } from './pages/teamstats/teamstats.component';
import { PlayerStatsModule } from './pages/playerstats/playerstats.module';
import { LoginComponent } from './pages/login/login.component';
import { LoginModule } from './pages/login/login.module';
import { TeamStatsModule } from './pages/teamstats/teamstats.module';
import { AuthGuardService } from 'src/services/auth-gaurd.service';

const routes: Routes = [
  {path: '', pathMatch: 'full', redirectTo: 'home', canActivate: [AuthGuardService]},
  {path: 'home', component: HomeComponent, canActivate: [AuthGuardService]},
  {path: 'playerstats/:playerId', component: PlayerStatsComponent, canActivate: [AuthGuardService]},
  {path: 'teamstats/:playerOneId/:playerTwoId', component: TeamStatsComponent, canActivate: [AuthGuardService]},
  {path: 'new/match/:gameType', component: NewMatchComponent, canActivate: [AuthGuardService]},
  {path: 'login', component: LoginComponent}
];

@NgModule({
  imports: [
    HomeModule,
    PlayerStatsModule,
    NewMatchModule,
    LoginModule,
    TeamStatsModule,
    RouterModule.forRoot(routes, {useHash: true})
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }
