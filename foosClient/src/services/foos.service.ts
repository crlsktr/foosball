import {Injectable} from '@angular/core';
import {HttpService} from './http.service';

@Injectable()
export class FoosService {

  constructor(private httpService: HttpService) {
  }

  public login() {
    return this.httpService.post(`/user/authenticate`, {username: 'bob', password: 'password'})
      .then((data) => {
        debugger;
      });
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

  public startGame(players) {
    if (players.length === 5) {
      return this.httpService.post('/gauntlet/create', {players: players.map(p => p.id)}, {withCredentials: true});
    } else {
      return this.httpService.post('/series/create', {players: players.map(p => p.id)}, {withCredentials: true});
    }
  }

}
