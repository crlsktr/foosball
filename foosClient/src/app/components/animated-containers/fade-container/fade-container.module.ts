import { NgModule } from '@angular/core';

import { FadeContainerComponent } from './fade-container.component';
import { BrowserModule } from '@angular/platform-browser';

@NgModule({
  imports: [
    BrowserModule,
  ],
  exports: [FadeContainerComponent],
  declarations: [FadeContainerComponent],
  providers: [],
})
export class FadeContainerModule { }
