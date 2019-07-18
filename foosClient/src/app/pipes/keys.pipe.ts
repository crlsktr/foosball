import { Pipe, PipeTransform } from '@angular/core';
import * as _ from 'lodash';

@Pipe({ name: 'keys' })
export class KeysPipe implements PipeTransform {
  public transform(val, args?: string[]): any {
    if (!val) {
      return val;
    }
    const items = [];
    for (const [key, value] of _.entries(val)) {
      items.push({ key, value });
    }
    return items;
  }
}
