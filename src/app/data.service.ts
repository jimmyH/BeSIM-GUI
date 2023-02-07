import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { environment } from '../environments/environment';

@Injectable({
  providedIn: 'root'
})
export class DataService {

  private REST_API_SERVER = environment.apiUrl;

  constructor(private http: HttpClient) { }

  public sendGetRequest(){
    return this.http.get(this.REST_API_SERVER);
  }

  getDevices(){
    return this.http.get(this.REST_API_SERVER + 'devices');
  }

  getDevice(id: String){
    return this.http.get(this.REST_API_SERVER + 'devices/' + id);
  }

  getRooms(deviceId: String){
    return this.http.get(this.REST_API_SERVER + 'devices/' + deviceId + '/rooms');
  }

  getRoom(deviceId: String, roomId: String){
    return this.http.get(this.REST_API_SERVER + 'devices/' + deviceId + '/rooms/' + roomId);
  }

  setT1(deviceId: String, roomId: String, val: String){
    const headers = { 'Content-Type' : 'application/json' };
    this.http.put(this.REST_API_SERVER + 'devices/' + deviceId + '/rooms/' + roomId + '/t1', val, { headers: headers })
        .subscribe();
  }
  setT2(deviceId: String, roomId: String, val: String){
    const headers = { 'Content-Type' : 'application/json' };
    this.http.put(this.REST_API_SERVER + 'devices/' + deviceId + '/rooms/' + roomId + '/t2', val, { headers: headers })
        .subscribe();
  }
  setT3(deviceId: String, roomId: String, val: String){
    const headers = { 'Content-Type' : 'application/json' };
    this.http.put(this.REST_API_SERVER + 'devices/' + deviceId + '/rooms/' + roomId + '/t3', val, { headers: headers })
        .subscribe();
  }

  setUnits(deviceId: String, roomId: String, val: String){
    const headers = { 'Content-Type' : 'application/json' };
    this.http.put(this.REST_API_SERVER + 'devices/' + deviceId + '/rooms/' + roomId + '/units', val, { headers: headers })
        .subscribe();
  }
  setAdvance(deviceId: String, roomId: String, val: String){
    const headers = { 'Content-Type' : 'application/json' };
    this.http.put(this.REST_API_SERVER + 'devices/' + deviceId + '/rooms/' + roomId + '/advance', val, { headers: headers })
        .subscribe();
  }
  setBoost(deviceId: String, roomId: String, val: String){
    const headers = { 'Content-Type' : 'application/json' };
    this.http.put(this.REST_API_SERVER + 'devices/' + deviceId + '/rooms/' + roomId + '/boost', val, { headers: headers })
        .subscribe();
  }
  setMode(deviceId: String, roomId: String, val: String){
    const headers = { 'Content-Type' : 'application/json' };
    this.http.put(this.REST_API_SERVER + 'devices/' + deviceId + '/rooms/' + roomId + '/mode', val, { headers: headers })
        .subscribe();
  }
  setSeason(deviceId: String, roomId: String, val: String){
    const headers = { 'Content-Type' : 'application/json' };
    this.http.put(this.REST_API_SERVER + 'devices/' + deviceId + '/rooms/' + roomId + '/winter', val, { headers: headers })
        .subscribe();
  }

  setProg(deviceId: String, roomId: String, dow: number, val: String){
    const headers = { 'Content-Type' : 'application/json' };
    this.http.put(this.REST_API_SERVER + 'devices/' + deviceId + '/rooms/' + roomId + '/days/' + dow, val, { headers: headers })
        .subscribe();
  }
}
