import { Component, OnInit, Input } from '@angular/core';
import { style, trigger, transition, animate, query, animateChild, keyframes } from '@angular/animations';

@Component({
  selector: 'app-flip-container',
  templateUrl: 'flip-container.component.html',
  styleUrls: ['flip-container.component.scss'],
  animations: [
    trigger('outer', [
      // state('true', style({height: '*', overflow: 'hidden'})),
      // state('false', style({height: '0', overflow: 'hidden'})),
      transition(':enter', [
        style({height: '0', overflow: 'hidden'}),
        animate('200ms ease-out', style({height: '*', overflow: 'visible'})),
        query('@inner', [
          animateChild(),
        ]),
      ]),
      transition(':leave', [
        style({height: '*', overflow: 'hidden'}),
        animate('300ms ease-out', style({height: '0', overflow: 'hidden'})),
      ]),
    ]),
    trigger('inner', [
      transition('* => true',
        animate('600ms',
          keyframes([
            style({transform: 'perspective(400px) rotate3d(1, 0, 0, 90deg)', opacity: 0, 'animation-timing-function': 'ease-in'}),
            style({transform: 'perspective(400px) rotate3d(1, 0, 0, -20deg)', opacity: 1, 'animation-timing-function': 'ease-in'}),
            style({transform: 'perspective(400px) rotate3d(1, 0, 0, 10deg)'}),
            style({transform: 'perspective(400px) rotate3d(1, 0, 0, -5deg)'}),
            style({transform: 'perspective(400px)'}),
          ]),
        ),
      ),
      transition('* => false',
        animate('600ms',
          keyframes([
            style({transform: 'perspective(400px)'}),
            style({transform: 'perspective(400px) rotate3d(1, 0, 0, -20deg)', opacity: 1}),
            style({transform: 'perspective(400px) rotate3d(1, 0, 0, 90deg)', opacity: 0}),
          ]),
        ),
      ),
    ]),
  ],
})
export class FlipContainerComponent implements OnInit {

  @Input() public open = false;

  constructor() { }

  ngOnInit() {
  }
}
