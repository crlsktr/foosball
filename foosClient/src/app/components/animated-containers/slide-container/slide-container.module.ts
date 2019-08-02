import { NgModule } from '@angular/core';

import { SlideContainerComponent } from './slide-container.component';
import { BrowserModule } from '@angular/platform-browser';

@NgModule({
  imports: [
    BrowserModule,
  ],
  exports: [SlideContainerComponent],
  declarations: [SlideContainerComponent],
  providers: [],
})
export class SlideContainerModule { }
