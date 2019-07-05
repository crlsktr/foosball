import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import {HttpService} from '../services/http.service';
import {FoosService} from '../services/foos.service';

@NgModule({
  declarations: [
    AppComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule
  ],
  providers: [
    HttpService,
    FoosService
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
