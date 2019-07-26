import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { PlayerStatsComponent } from './playerstats.component';
import {PipesModule} from '../../pipes/pipes.module';

@NgModule({
  declarations: [PlayerStatsComponent],
  imports: [
    CommonModule,
    PipesModule,
  ]
})
export class PlayerStatsModule { }
