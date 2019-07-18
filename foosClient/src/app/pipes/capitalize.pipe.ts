import {Pipe, PipeTransform} from '@angular/core';
import * as _ from 'lodash';

@Pipe({ name: 'capitalize' })
export class CapitalizePipe implements PipeTransform {
  public transform(value, args?) {
    return _.startCase(_.lowerCase(value));
  }
}
