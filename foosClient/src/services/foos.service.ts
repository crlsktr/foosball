import {Injectable} from '@angular/core';
import {HttpService} from './http.service';

@Injectable()
export class FoosService {

  constructor(private httpService: HttpService) {
  }

  public getOverallStats() {
    this.httpService.get('/api/stats')
      .then((data) => {
        return data;
      });
  }
}
