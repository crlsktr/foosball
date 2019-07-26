import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { HomeComponent } from './home.component';
import {RouterModule} from '@angular/router';
import { TypeAheadModule } from 'src/app/components/type-ahead/type-ahead.module';

@NgModule({
  declarations: [HomeComponent],
  imports: [
    CommonModule,
    RouterModule,
    TypeAheadModule,
  ]
})
export class HomeModule {}
