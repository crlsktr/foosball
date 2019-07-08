import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NewMatchComponent } from './new-match.component';
import {FormsModule} from '@angular/forms';

@NgModule({
  declarations: [NewMatchComponent],
  imports: [
    CommonModule,
    FormsModule,
  ]
})
export class NewMatchModule { }
