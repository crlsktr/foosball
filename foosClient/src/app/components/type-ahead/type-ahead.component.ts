import {
  Component, EventEmitter, Input, OnDestroy, OnInit, Output,
  ViewChild, forwardRef, ChangeDetectionStrategy, ChangeDetectorRef
} from '@angular/core';
import { Subscription, Observable, fromEvent } from 'rxjs';
import * as _ from 'lodash';


import { ControlValueAccessor, NG_VALUE_ACCESSOR } from '@angular/forms';
import { filter, map } from 'rxjs/operators';


@Component({
  selector: 'app-type-ahead',
  templateUrl: './type-ahead.component.html',
  styleUrls: ['./type-ahead.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  providers: [
    {
      provide: NG_VALUE_ACCESSOR,
      useExisting: forwardRef(() => TypeAheadComponent),
      multi: true,
    }
  ],
})
export class TypeAheadComponent implements OnInit, OnDestroy, ControlValueAccessor {

  public predictions: any[] = [];
  public isFocused = false;
  public highlightItemIndex = 0;
  @Input() public borderRadius;
  @Input() public prependIcon;
  @Input() public autoFocus = false;
  @Input() public placeholder: string;
  @Input() public key: string;
  @Input() public secondaryKey: string;
  @Input() public dropDown = false;
  @Input() public canCreate = false;
  @Input() public useOnlyOptions = false;
  @Input() public disabled = false;
  // Enable this to automatically select the item if the input box is blurred.
  @Input() public selectOnBlur = false;
  public get options() { return this._options; }
  @Input() public set options(val) {
    this._options = val;
    this.onOptionsChanged();
  }

  public get selectedItem() { return this._selectedItem; }
  @Input() public set selectedItem(val) {
    this._selectedItem = val;
    if (val || val === 0) {
      this.input.nativeElement.blur();
      if (this.key) {
        this.input.nativeElement.value = val[this.key] !== undefined ? val[this.key] : null;
      } else {
        this.input.nativeElement.value = val;
      }
    } else {
      this.input.nativeElement.value = null;
    }
  }

  @Output() public selectedItemChange = new EventEmitter();
  @Output() public inputChange = new EventEmitter<string>();
  @Output() public enterPressed = new EventEmitter();

  @ViewChild('input', {static: true}) public input;
  @ViewChild('dropDownComponent', {static: true}) public dropDownComponent;
  @ViewChild('dropDownItem', {static: true}) public dropDownItem;

  private input$: Subscription;
  private focus$: Subscription;
  private blur$: Subscription;
  private arrows$: Subscription;

  private _selectedItem: any;
  private _options: any[] = [];
  private subscribers = [];
  private propagateChange = (val: any) => { };
  private propagateTouch = () => { };

  constructor(
    private cd: ChangeDetectorRef,
  ) { }

  public ngOnInit() {
    this.input$ = fromEvent(this.input.nativeElement, 'keyup')
      .pipe(
        filter((e: any) => e.code !== 'ArrowDown' && e.code !== 'ArrowUp'),
        map((i: any) => i.currentTarget.value),
      ).subscribe((input) => {
        this.onInputChanged(input);
        this.inputChange.emit(input);
      });
    this.focus$ = fromEvent(this.input.nativeElement, 'focus').subscribe((e) => {
      this.isFocused = true;
      if (this.dropDown && !this.input) {
        this.onInputChanged('');
      }
      this.cd.markForCheck();
      this.propagateTouch();
    });
    this.blur$ = fromEvent(this.input.nativeElement, 'blur').subscribe((e) => {
      this.isFocused = false;
      this.cd.markForCheck();
    });
    this.arrows$ = fromEvent(this.input.nativeElement, 'keydown').subscribe((e: any) => {
      if (e.code === 'ArrowDown') {
        e.preventDefault();
        this.onDownArrow();
      } else if (e.code === 'ArrowUp') {
        e.preventDefault();
        this.onUpArrow();
      } else if (e.code === 'Enter' || e.code === 'Tab') {
        if (this.highlightItemIndex > -1) {
          this.predictionSelected(this.predictions[this.highlightItemIndex], e);
        }
      }
      this.cd.markForCheck();
    });

    if (this.autoFocus) {
      setTimeout(() => {
        this.input.nativeElement.focus();
      });
    }

    this.subscribers.push(this.input$, this.focus$, this.blur$, this.arrows$);
  }

  public writeValue(val: any) {
    this.selectedItem = val;
  }

  public registerOnChange(fn: any) {
    this.propagateChange = fn;
  }

  public registerOnTouched(fn: any) {
    this.propagateTouch = fn;
  }

  public setDisabledState(isDisabled: boolean) {
    this.disabled = isDisabled;
    this.cd.markForCheck();
  }


  public getRawInput() {
    return this.input.nativeElement.value;
  }

  public ngOnDestroy() {
    _.forEach(this.subscribers, (sub) => {
      sub.unsubscribe();
    });
  }

  public onBlur() {
    if (this.selectOnBlur && !this.selectedItem && (this.highlightItemIndex || this.highlightItemIndex === 0) && this.predictions) {
      this.predictionSelected(this.predictions[this.highlightItemIndex], null);
    }
  }
  public predictionSelected(p, event) {
    if (p) {
      if (event) {
        event.preventDefault();
      }
      this.selectedItem = p.item;
      this.propagateChange(this.selectedItem);
      this.selectedItemChange.emit(this.selectedItem);
      this.isFocused = false;
    }
  }

  private onOptionsChanged() {
    this.onInputChanged(this.input.nativeElement.value);
  }

  private onInputChanged(rawInput) {
    this.highlightItemIndex = 0;
    const input = rawInput.toLowerCase();
    if (input === '' && this.selectedItem) {
      this.selectedItem = null;
      this.propagateChange(this.selectedItem);
      this.selectedItemChange.emit(this.selectedItem);
    }
    if (!rawInput && !this.dropDown) {
      this.predictions = [];
    } else if (this.options) {
      this.predictions = _.sortBy(this.options.filter((o) => {
        if (this.useOnlyOptions) {
          return true;
        }
        const val = _.toString(this.key ? o[this.key] : o);
        if (!input || val.toLowerCase().indexOf(input) !== -1) {
          return true;
        }
        const secondaryVal = _.toString(this.secondaryKey ? o[this.secondaryKey] : null);
        return secondaryVal && secondaryVal.toLowerCase().indexOf(input) !== -1;
      }).map((o) => {
        const val = _.toString(this.key ? o[this.key] : o);
        const matches = [];
        let index = -1;
        let search = val;
        if (input && search) {
          do {
            index = search.toLowerCase().indexOf(input);
            if (index !== -1) {
              matches.push({
                offset: (val.length - search.length) + index,
                length: input.length,
              });
              search = search.substr((index + 1), search.length - (index + 1));
            }
          } while (index !== -1);
        }

        const secondaryVal = _.toString(this.secondaryKey ? o[this.secondaryKey] : null);
        const secondaryMatches = [];
        index = -1;
        search = secondaryVal;
        if (input && search) {
          do {
            index = search.toLowerCase().indexOf(input);
            if (index !== -1) {
              secondaryMatches.push({
                offset: (secondaryVal.length - search.length) + index,
                length: input.length,
              });
              search = search.substr((index + 1), search.length - (index + 1));
            }
          } while (index !== -1);
        }

        return {
          description: val,
          structured_formatting: {
            main_match_count: matches.length,
            main_text: val,
            main_parts: this.getMatchingParts(val, matches),
            secondary_text: secondaryVal,
            secondary_parts: secondaryVal ? this.getMatchingParts(secondaryVal, secondaryMatches) : null,
            secondary_match_count: secondaryMatches.length,
          },
          item: o,
        };
      }), [
          (o) => {
            return 0 - (o.structured_formatting.main_match_count * 2) - (o.structured_formatting.secondary_match_count);
          },
        ]);
      if (this.canCreate && input.length > 0) {
        let item = {};
        if (this.key) {
          item[this.key] = rawInput;
        } else {
          item = rawInput;
        }
        let hasItem = false;    // added this to account for the "create new" coming up on already created values issue
        _.forEach(this.predictions, (p) => {
          if (_.toString(p.item) === item) {
            hasItem = true;
          }
        });
        if (!hasItem) {
          this.predictions.splice(0, 0, {
            item,
            description: rawInput,
            structured_formatting: {
              main_match_count: 1,
              main_text: rawInput,
              main_parts: this.getMatchingParts(rawInput + '  (Create New)', [{ offset: 0, length: input.length }])
            },
          });
        }
      }
    }
    this.cd.markForCheck();
  }

  private onDownArrow() {
    this.highlightItemIndex++;
    if (this.highlightItemIndex > this.predictions.length - 1) {
      this.highlightItemIndex = this.predictions.length - 1;
    } else {
      this.dropDownComponent.nativeElement.scrollTop += this.dropDownItem.nativeElement.offsetHeight;
    }
  }

  private onUpArrow() {
    this.highlightItemIndex--;
    if (this.highlightItemIndex < 0) {
      this.highlightItemIndex = 0;
    } else {
      this.dropDownComponent.nativeElement.scrollTop -= this.dropDownItem.nativeElement.offsetHeight;
    }
  }

  private getMatchingParts(str, matches = []) {
    const parts = [];
    if (matches.length === 0) {
      parts.push({
        text: str,
      });
    } else {
      matches.forEach((match, i) => {
        if (i === 0) {
          if (match.offset !== 0) {
            parts.push({
              text: str.substr(0, match.offset),
            });
          }
        }

        parts.push({
          text: str.substr(match.offset, match.length),
          match: true,
        });

        const next = matches[i + 1];
        parts.push({
          text: str.substr(match.offset + match.length,
            (next ? next.offset : str.length) - (match.offset + match.length)),
        });
      });
    }
    return parts;
  }

}
