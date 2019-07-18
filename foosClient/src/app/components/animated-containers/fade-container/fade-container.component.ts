import { Component, OnInit, Input } from '@angular/core';
import { style, trigger, transition, animate } from '@angular/animations';

@Component({
  selector: 'app-fade-container',
  templateUrl: './fade-container.component.html',
  styleUrls: ['./fade-container.component.scss'],
  animations: [
    trigger('fade', [
      transition(':enter', [
        style({height: '0', opacity: 0}),
        animate('200ms ease-out', style({height: '*', opacity: 1})),
      ]),
      transition(':leave', [
        style({height: '*', opacity: 1}),
        animate('200ms ease-out', style({height: '0', opacity: 0})),
      ]),
    ]),
  ],
})
export class FadeContainerComponent implements OnInit {
  @Input() open = false;

  constructor() { }

  ngOnInit() { }
}
