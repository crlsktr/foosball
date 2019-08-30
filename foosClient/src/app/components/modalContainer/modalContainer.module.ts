import { ModalActionsComponent } from './modalActions/modalActions.component';
import { ModalContentComponent } from './modalContent/modalContent.component';
import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { ModalContainerComponent } from './modalContainer.component';
import { ModalTitleComponent } from './modalTitle/modalTitle.component';

@NgModule({
    imports: [
        BrowserModule,
    ],
    exports: [
        ModalContainerComponent,
        ModalTitleComponent,
        ModalContentComponent,
        ModalActionsComponent,
    ],
    declarations: [
        ModalContainerComponent,
        ModalTitleComponent,
        ModalContentComponent,
        ModalActionsComponent,
    ],
    providers: [],
})
export class ModalContainerModule { }
