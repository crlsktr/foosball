import { ModalTitleComponent } from './modalTitle/modalTitle.component';
import { Component, OnInit, ContentChild, AfterContentInit, Output, EventEmitter, OnDestroy, Input } from '@angular/core';

@Component({
    selector: 'app-modal-container',
    templateUrl: 'modalContainer.component.html',
    styleUrls: ['modalContainer.component.scss'],
})
export class ModalContainerComponent implements OnInit, OnDestroy, AfterContentInit {

    @ContentChild(ModalTitleComponent, {static: false}) public titleComponent: ModalTitleComponent;
    @Input() public minWidth;
    @Output() public close = new EventEmitter();

    private subscriptions = [];

    constructor() { }

    ngOnInit() { }

    ngOnDestroy() {
        this.subscriptions.forEach((s) => s.unsubscribe());
    }

    ngAfterContentInit() {
        this.subscriptions.push(this.titleComponent.close.subscribe(() => {
            this.close.next();
        }));
    }

}
