import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TeamStatsComponent } from './teamstats.component';
import { RouterModule } from '@angular/router';
import { TimeSeriesModule } from 'src/app/components/time-series/time-series.module';

@NgModule({
  declarations: [TeamStatsComponent],
  imports: [
    CommonModule,
    RouterModule,
    TimeSeriesModule,
  ]
})
export class TeamStatsModule { }
