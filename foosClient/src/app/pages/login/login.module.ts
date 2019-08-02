import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LoginComponent } from './login.component';
import {RouterModule} from '@angular/router';
import { TypeAheadModule } from 'src/app/components/type-ahead/type-ahead.module';

import { FormsModule } from '@angular/forms';
import {FlipContainerModule} from '../../components/animated-containers/flip-container/flip-container.module';

@NgModule({
  declarations: [LoginComponent],
  imports: [
    CommonModule,
    RouterModule,
    TypeAheadModule,
    FormsModule,
    FlipContainerModule,
  ]
})
export class LoginModule {}
