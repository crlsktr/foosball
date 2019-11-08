import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TeamStatsComponent } from './teamstats.component';
import { RouterModule } from '@angular/router';

@NgModule({
  declarations: [TeamStatsComponent],
  imports: [
    CommonModule,
    RouterModule,
  ]
})
export class TeamStatsModule { }
