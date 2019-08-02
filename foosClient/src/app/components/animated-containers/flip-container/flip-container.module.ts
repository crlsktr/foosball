import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { FlipContainerComponent } from './flip-container.component';

@NgModule({
  imports: [
    BrowserModule,
  ],
  exports: [FlipContainerComponent],
  declarations: [FlipContainerComponent],
  providers: [],
})
export class FlipContainerModule { }
