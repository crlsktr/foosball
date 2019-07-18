import { Component, OnInit, Input } from '@angular/core';
import { style, trigger, transition, animate } from '@angular/animations';

@Component({
  selector: 'app-slide-container',
  templateUrl: './slide-container.component.html',
  styleUrls: ['./slide-container.component.scss'],
  animations: [
    trigger('slide', [
      transition(':enter', [
        style({height: '0', overflow: 'hidden'}),
        animate('200ms ease-out', style({height: '*', overflow: 'hidden'})),
      ]),
      transition(':leave', [
        style({height: '*', overflow: 'hidden'}),
        animate('200ms ease-out', style({height: '0', overflow: 'hidden'})),
      ]),
    ]),
  ],
})
export class SlideContainerComponent implements OnInit {
  @Input() open = false;

  constructor() { }

  ngOnInit() { }
}
