import { Pipe, PipeTransform } from '@angular/core';

import * as _ from 'lodash';

@Pipe({ name: 'orderBy' })
export class OrderByPipe implements PipeTransform  {
  public transform(array, args?) {
    if (args) {
      return _.sortBy(array, args);
    } else {
      return _.sortBy(array);
    }
  }
}
