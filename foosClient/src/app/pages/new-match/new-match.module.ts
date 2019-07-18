import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NewMatchComponent } from './new-match.component';
import {FormsModule} from '@angular/forms';
import {TypeAheadModule} from '../../components/type-ahead/type-ahead.module';
import {CheckboxInputModule} from '../../components/checkbox-input/checkbox-input.module';

@NgModule({
  declarations: [NewMatchComponent],
  imports: [
    CommonModule,
    FormsModule,
    TypeAheadModule,
    CheckboxInputModule,
  ]
})
export class NewMatchModule { }
