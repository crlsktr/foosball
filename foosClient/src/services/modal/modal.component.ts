import {
  Component, ComponentFactoryResolver, ComponentRef, Output, ViewChild, ViewContainerRef, EventEmitter, OnInit,
  OnDestroy
} from '@angular/core';
import { animate, state, style, transition, trigger } from '@angular/animations';
import { IModalComponent, IModalOptions } from './modal.service';
import { Subscription } from 'rxjs';
import { take } from 'rxjs/operators';

@Component({
  selector: 'app-modal-component',
  templateUrl: './modal.component.html',
  styleUrls: ['./modal.component.scss'],
  animations: [
    trigger('fadeInAnimation', [
      transition(':enter', [
        style({ opacity: 0 }),
        animate('.4s ease-out', style({ opacity: 1 }))]),
    ]),
    trigger('slideInOut', [
      state('for', style({
        transform: '{{transform}}'
      }), {params: {transform: 'translate(70%, -65%) scale(.05, .05)'}}),
      transition('void => state1', [
        style({ transform: 'translate(0, -50%)' }),
        animate('.4s ease-out', style({ transform: 'translate(0, 0)' }))]),
      transition('void => guide', [
        style({ transform: '{{transform}}', opacity: .5 }),
        animate('.4s ease-out', style({ transform: 'translate(0, 0)' }))]),
      transition('state1 => state2', [
        animate('.4s ease-out', style({ transform: 'translate(0, -100%)', opacity: 0 }))]),
      transition('guide => state2', [
        animate('.42s ease-out', style({ transform: '{{transform}}', opacity: .5}))]),
    ])],
})
export class ModalComponent implements OnInit, OnDestroy {

  @ViewChild('content', { read: ViewContainerRef, static: true }) public content;
  @Output() public onDismiss = new EventEmitter();
  @Output() public onTryToDismiss = new EventEmitter();

  public transform;
  public animationState = 'state1';
  private componentRef: ComponentRef<any>;
  private options: IModalOptions = {};
  private subscriptions: Subscription[] = [];

  constructor(
    private resolver: ComponentFactoryResolver,
  ) {
  }

  public ngOnInit() {
  }

  public ngOnDestroy() {
    this.subscriptions.forEach((s) => s.unsubscribe());
  }

  public setContentComponent(component: IModalComponent, options: IModalOptions = {}) {
    if (options.animateGuide) {
      this.animationState = 'guide';
      const profileWidth = document.getElementById('rootProfile').offsetWidth;
      this.transform = `translate(${(window.innerWidth / 2) - (profileWidth + 40)}px, -65%) scale(.05, .05)`;
    }
    this.content.clear();
    this.componentRef = this.content.createComponent(this.resolver.resolveComponentFactory(component as any));
    this.componentRef.instance.onDismiss.pipe(take(1)).subscribe((event) => {
      this.dismiss(event);
    });
    this.options = options;
    this.componentRef.instance.params = this.options.params;
    if (this.componentRef.instance.onCloseClicked) {
      this.subscriptions.push(this.componentRef.instance.onCloseClicked.subscribe((event) => {
        if (this.componentRef.instance.hasUnsavedChanges) {
          this.onTryToDismiss.next();
        } else {
          this.dismiss();
        }
      }));
    }
  }

  public backdropClicked(event) {
    if (!this.options.disableBackdropDismiss && this.componentRef.instance.modalBackdropClicked) {
      this.componentRef.instance.modalBackdropClicked();
    } else if (!this.options.disableBackdropDismiss) {
      if (this.componentRef.instance.hasUnsavedChanges) {
        this.onTryToDismiss.next();
      } else {
        this.dismiss();
      }
    }
  }
  public dismiss(event?) {
    this.animationState = 'state2';
    this.onDismiss.emit(event);
  }
}
