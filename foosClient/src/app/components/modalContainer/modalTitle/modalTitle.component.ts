import { EventEmitter, Component, OnInit, Output } from '@angular/core';

@Component({
    selector: 'app-modal-title',
    templateUrl: 'modalTitle.component.html',
    styleUrls: ['modalTitle.component.scss'],
})
export class ModalTitleComponent implements OnInit {
    @Output() public close = new EventEmitter();

    constructor() { }

    ngOnInit() { }
}
