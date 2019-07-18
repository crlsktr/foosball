import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { CheckboxInputComponent } from './checkbox-input.component';

@NgModule({
  imports: [
    BrowserModule,
    FormsModule,
    ReactiveFormsModule,
  ],
  exports: [CheckboxInputComponent],
  declarations: [CheckboxInputComponent],
  providers: [],
})
export class CheckboxInputModule { }
