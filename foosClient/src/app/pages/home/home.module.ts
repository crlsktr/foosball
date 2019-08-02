import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { HomeComponent } from './home.component';
import {RouterModule} from '@angular/router';
import { TypeAheadModule } from 'src/app/components/type-ahead/type-ahead.module';
import { FormsModule } from '@angular/forms';

@NgModule({
  declarations: [HomeComponent],
  imports: [
    CommonModule,
    RouterModule,
    TypeAheadModule,
    FormsModule,
  ]
})
export class HomeModule {}
