import {Injectable} from '@angular/core';
import {HttpService} from './http.service';
import {GameResult} from '../app/pages/new-match/new-match.component';

@Injectable()
export class FoosService {

  constructor(private httpService: HttpService) {
  }

  public login() {
    return this.httpService.post(`/user/authenticate`, {username: 'daniel', password: 'password'}, {withCredentials: true});
  }

  public getAllPlayers() {
    return this.httpService.get('/player/search/all/100000')
      .then((data) => {
        if (data && data.Ok) {
          return data.Ok;
        }
      });
  }

  public getAllUsers() {
    return this.httpService.get('/user/search/all')
      .then((data) => {
        if (data && data.Ok) {
          return data.Ok;
        }
      });
  }

  public searchUser(searchTerm) {
    return this.httpService.post('/user/search', {term: searchTerm})
      .then((data) => {
        // todo: parse data
        return data;
      });
  }

  public startGame(players) {
    if (players.length === 5) {
      return this.httpService.post('/gauntlet/create', {players: players.map(p => p.id)}, {withCredentials: true})
        .then((data) => {
          if (data && data.Ok) {
            return data.Ok;
          }
        });

    } else {
      return this.httpService.post('/series/create', {players: players.map(p => p.id)}, {withCredentials: true})
        .then((data) => {
          if (data && data.Ok) {
            return data.Ok;
          }
        });
    }
  }

  public finishGame(gameResults: GameResult[]) {
    return this.httpService.post(`/game/finish`, {game_results: gameResults}, {withCredentials: true});
  }

  public loadLeaderboard() {
    return this.httpService.get(`/report/leaderboard`, {withCredentials: true})
      .then((data) => {
        if (data && data.Ok) {
          return data.Ok;
        }
      });
  }

  public getPlayersStats(playerId: number) {
    return this.httpService.get(`/report/playerstats/${playerId}`)
      .then((data) => {
        if (data && data.Ok) {
          return data.Ok;
        }
      });
  }
}
