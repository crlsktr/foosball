import {NgModule} from '@angular/core';

import {TypeAheadComponent} from './type-ahead.component';
import {BrowserModule} from '@angular/platform-browser';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';

@NgModule({
  imports: [
    BrowserModule,
    FormsModule,
    ReactiveFormsModule,
  ],
  exports: [TypeAheadComponent],
  declarations: [TypeAheadComponent],
  entryComponents: [TypeAheadComponent],
  providers: [],
})
export class TypeAheadModule {
}
