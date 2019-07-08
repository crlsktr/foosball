import {Injectable} from '@angular/core';
import {HttpService} from './http.service';

@Injectable()
export class FoosService {

  constructor(private httpService: HttpService) {
  }

  public getAllUsers() {
    return this.httpService.get('/user/search/all')
      .then((data) => {
        if (data && data.Ok) {
          return data.Ok;
        }
      })
  }
  public searchUser(searchTerm) {
    return this.httpService.post('/user/search', {term: searchTerm})
      .then((data) => {
        //todo: parse data
        return data;
      })

  }

}
