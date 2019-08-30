import { take } from 'rxjs/operators';
import {
  ApplicationRef, ComponentFactoryResolver, EmbeddedViewRef, EventEmitter, Injectable, Injector,
} from '@angular/core';
import { ModalComponent } from './modal.component';


export interface IModalOptions {
  params?: any;
  disableBackdropDismiss?: boolean;
  animateGuide?: boolean;
}

export interface IModalComponent {
  onDismiss: EventEmitter<any>;
  onCloseClicked?: EventEmitter<any>;
  params?: any;
  hasUnsavedChanges?: boolean;
  modalBackdropClicked?: () => void;
}

@Injectable()
export class ModalService {

  public get isModalOpen() { return this.openModalCount > 0; }

  private openModalCount = 0;
  private openModals = [] as any[];
  private openSubscriptions = [];
  constructor(
    private componentFactoryResolver: ComponentFactoryResolver,
    private appRef: ApplicationRef,
    private injector: Injector,
  ) { }

  public dismissAll() {
    this.openModals.forEach((m) => {
      this.appRef.detachView(m);
    });
    this.openModals = [];
  }

  public create(component: any, options: IModalOptions = {}): Promise<any> {
    this.openModalCount++;
    return new Promise((resolve) => {
      // 1. Create a component reference from the component
      const componentRef = this.componentFactoryResolver
        .resolveComponentFactory(ModalComponent)
        .create(this.injector);
      componentRef.instance.setContentComponent(component, options);
      componentRef.instance.onDismiss.pipe(take(1)).subscribe((event) => {
        setTimeout(() => {
          this.openModalCount--;
          this.appRef.detachView(componentRef.hostView);
          const index = this.openModals.indexOf(componentRef.hostView);
          this.openModals.splice(index, 1);
          componentRef.destroy();
          resolve(event);
        }, 400);
      });

      // 2. Attach component to the appRef so that it's inside the ng component tree
      this.appRef.attachView(componentRef.hostView);
      this.openModals.push(componentRef.hostView);
      // 3. Get DOM element from component
      const domElem = (componentRef.hostView as EmbeddedViewRef<any>)
        .rootNodes[0] as HTMLElement;

      // 4. Append DOM element to the body
      document.body.appendChild(domElem);

      setTimeout(() => {
        const focusedElement: any = document.querySelector(':focus');
        if (focusedElement && focusedElement.blur) {
          focusedElement.blur();
        }
      });
    });
  }



}
