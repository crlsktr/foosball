import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LoginComponent } from './login.component';
import {RouterModule} from '@angular/router';
import { TypeAheadModule } from 'src/app/components/type-ahead/type-ahead.module';

import { FormsModule } from '@angular/forms';

@NgModule({
  declarations: [LoginComponent],
  imports: [
    CommonModule,
    RouterModule,
    TypeAheadModule,
    FormsModule,
  ]
})
export class LoginModule {}
