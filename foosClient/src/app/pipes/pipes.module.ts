import {NgModule} from '@angular/core';
import {OrderByPipe} from './orderBy.pipe';
import {CapitalizePipe} from './capitalize.pipe';
import {KeysPipe} from './keys.pipe';

@NgModule({
  declarations: [
    OrderByPipe,
    CapitalizePipe,
    KeysPipe,
  ],
  exports: [
    OrderByPipe,
    CapitalizePipe,
    KeysPipe,
  ],
})
export class PipesModule {
}
