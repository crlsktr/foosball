import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { HomeComponent } from './home.component';
import {RouterModule} from '@angular/router';
import { TypeAheadModule } from 'src/app/components/type-ahead/type-ahead.module';
import { FormsModule } from '@angular/forms';
import { LeaderBoardsModule } from 'src/app/components/leaderboards/leaderboards.module';

@NgModule({
  declarations: [HomeComponent],
  imports: [
    CommonModule,
    RouterModule,
    TypeAheadModule,
    FormsModule,
    LeaderBoardsModule
  ]
})
export class HomeModule {}
