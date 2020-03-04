import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TimeSeriesComponent } from './time-series.component';



@NgModule({
  declarations: [
    TimeSeriesComponent
  ],
  imports: [
    CommonModule
  ],
  exports: [
    TimeSeriesComponent
  ]
})
export class TimeSeriesModule { }
