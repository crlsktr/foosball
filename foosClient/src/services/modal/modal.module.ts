import {NgModule} from '@angular/core';
import {ModalService} from './modal.service';
import {ModalComponent} from './modal.component';
import {BrowserModule} from '@angular/platform-browser';
import {ModalContainerModule} from '../../app/components/modalContainer/modalContainer.module';

@NgModule({
  imports: [
    ModalContainerModule,
    BrowserModule,
  ],
  declarations: [
    ModalComponent,
  ],
  entryComponents: [
    ModalComponent,
  ],
  exports: [],
  providers: [
    ModalService,
  ],
})
export class ModalModule {}
