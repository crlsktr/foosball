import {Injectable} from '@angular/core';
import {HttpService} from './http.service';
import {HttpClient, HttpHeaders} from '@angular/common/http';

@Injectable()
export class FoosService {

  constructor(private httpService: HttpService, private httpClient: HttpClient) {
  }

  public getOverallStats() {
    this.httpService.get('/api/stats')
      .then((data) => {
        return data;
      });
  }

  public searchUser() {
    debugger;
    return this.httpService.post('/user/search', {term: 'dan'})
      .then((data) => {
        //todo: parse data
        return data;
      })

  }

  public searchUser2() {
    debugger;
    this.httpService.get('/user/search/cad')
      .then((data) => {
        debugger;
      })
      .catch((err) => {
        debugger;
      });
  }
}
