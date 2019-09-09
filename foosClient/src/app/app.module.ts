import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import {HttpService} from '../services/http.service';
import {FoosService} from '../services/foos.service';
import {HttpClientModule} from '@angular/common/http';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';
import {SocketService} from '../services/socket.service';
import {ModalService} from '../services/modal/modal.service';
import {ModalModule} from '../services/modal/modal.module';
import { AuthGuardService } from 'src/services/auth-gaurd.service';

@NgModule({
  declarations: [
    AppComponent
  ],
  imports: [
    HttpClientModule,
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    ModalModule,
  ],
  providers: [
    HttpService,
    FoosService,
    SocketService,
    ModalService,
    AuthGuardService,
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
