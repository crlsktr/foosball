import {Component, Input, OnInit} from '@angular/core';

@Component({
    selector: 'app-modal-content',
    templateUrl: 'modalContent.component.html',
    styleUrls: ['modalContent.component.scss'],
})
export class ModalContentComponent implements OnInit {
  @Input() public minWidth;
  @Input() public minHeight;
  @Input() overflowVisible = false;

    constructor() { }

    ngOnInit() { }

}
