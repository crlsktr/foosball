import {ChangeDetectionStrategy, Component, EventEmitter, forwardRef, Input, OnInit, Output} from '@angular/core';
import {ControlValueAccessor, NG_VALUE_ACCESSOR} from '@angular/forms';

@Component({
  selector: 'app-checkbox-input',
  templateUrl: 'checkbox-input.component.html',
  styleUrls: ['checkbox-input.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  providers: [
    {
      provide: NG_VALUE_ACCESSOR,
      useExisting: forwardRef(() => CheckboxInputComponent),
      multi: true,
    },
  ],
})
export class CheckboxInputComponent implements OnInit, ControlValueAccessor {

  @Input() public text: string;
  @Input() public isChecked = false;
  @Input() isDisabled = false;
  @Input() checkBoxColor = '#f3ae2b';
  @Output() public toggleChange = new EventEmitter<boolean>();

  private propagateChange = (val: boolean) => {};

  constructor() { }

  public ngOnInit() { }

  public toggle() {
    if (!this.isDisabled) {
      this.isChecked = !this.isChecked;
      this.propagateChange(this.isChecked);
      this.toggleChange.emit(this.isChecked);
    }
  }

  public writeValue(value: any) {
    this.isChecked = !!value;
  }

  public registerOnChange(fn: any) {
    this.propagateChange = fn;
  }

  public setDisabledState(isDisabled: boolean) {
    this.isDisabled = isDisabled;
  }

  public registerOnTouched(fn: any) { }

}
