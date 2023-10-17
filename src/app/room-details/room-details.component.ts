import { Component, OnInit, ViewChild } from '@angular/core';
import { DataService } from '../data.service';
import { ActivatedRoute, Router } from '@angular/router';
import { GoogleChartComponent, ChartType, ChartSelectionChangedEvent } from 'angular-google-charts';
import { MatSelectChange } from '@angular/material/select';
import * as moment from 'moment';

interface Day {
  value: number;
  viewValue: string;
};

@Component({
  selector: 'app-room-details',
  templateUrl: './room-details.component.html',
  styleUrls: ['./room-details.component.css']
})

export class RoomDetailsComponent implements OnInit {

  days : Day[] = [
    { value: 0, viewValue: 'Sunday' },
    { value: 1, viewValue: 'Monday' },
    { value: 2, viewValue: 'Tuesday' },
    { value: 3, viewValue: 'Wednesday' },
    { value: 4, viewValue: 'Thursday' },
    { value: 5, viewValue: 'Friday' },
    { value: 6, viewValue: 'Saturday' },
  ];
  selectedDay : number = (new Date()).getDay();

  selectDay(event: MatSelectChange) {
    this.updateGraph(true);
  }



  type: ChartType = ChartType.ColumnChart;  
  data = [ [ "0:00", 0 ] ] // Array<Array<string | int>>
  columnNames = ['Time', 'Temp'];  
  options = {      
    'legend' : { 'position': 'none' },
    'vAxis' :  { 'textPosition': 'none',
                 'gridlines' : { 'color': 'transparent' },
                 'ticks' : [ 0, 1, 2, 3 ]
               },
    'hAxis' :  { 'textPosition': 'none',
               },
    'backgroundColor' : 'transparent',
    'bar' : { 'groupWidth' : '90%' },
    'chartArea' : { 'width' : '100%', 'height' : '70%' },
    'width' : '100%',
    'height' : '100%',
  };  

  onChartSelect(event: ChartSelectionChangedEvent) {
    if (event['selection']['length']>0){
      console.log(event);
      const { row, column } = event['selection'][0];
      if (row != undefined){
        let time : moment.Moment = this.getProgTime(row);
        let dow : number = this.selectedDay;
        let val : number = this.getProg( dow, row); // 0..2
        let timeStr : string = time.format('H:mm');
        console.log(`${timeStr} ${val}`);
        this.updateProg(dow, row, (val+1)%3);
        
        this.updateGraph(true);
      }
    }
  }

  // Converts { hours, minutes } to a 30min index into the day 0..47
  getProgIdx(time: moment.Moment): number{
      return 2*time.hour() + Math.floor(time.minute()/30);
  }

  getProgTime(idx: number): moment.Moment{
      let hr: number = Math.floor(idx/2);
      let min: number = (idx%2)*30; 
      return moment({ hours: hr, minute: min});
  }

  // 
  getProg(dow: number, _idx: number): number{
    let idx : number = Math.floor(_idx/2);
    let nibble : number = _idx%2;
    let val : number = this.room.days[dow][idx]; // @todo Need to check dow=0 is at days[0] etc.
    if ( nibble==0 ){
      return val & 0x0f;
    } else {
      return (val>>4) & 0x0f;
    }
  }

  updateProg(dow: number, _idx: number, new_val: number): void{
    let idx : number = Math.floor(_idx/2);
    let nibble : number = _idx%2;
    let val : number = this.room.days[dow][idx]; // @todo Need to check dow=0 is at days[0] etc.
    if ( nibble==0 ){
      this.room.days[dow][idx] = (val & 0xf0) | new_val;
    } else {
      this.room.days[dow][idx] = (val & 0x0f) | (new_val<<4);
    }
    console.log(`dow ${dow} idx ${idx} new value ${this.room.days[dow][idx]} old value ${val}`);
    console.log(this.room.days[dow]);

    this.setProg(dow,this.room.days[dow]);
  }


  getTemp(val: number): number{
    if (val==0){
      return this.room.t1/10; // @todo conversion to degF
    } else if (val==1){
      return this.room.t2/10;
    } else if (val==2){
      return this.room.t3/10;
    }
    return 0;
  }

  updateGraph(forceRefresh: boolean = false): void{
    let dow : number = this.selectedDay;
    while (this.data.length>0) { this.data.pop(); }
    for(let hour = 0; hour < 24; hour++) {
      let time : moment.Moment;
      let val : number;

      time = moment({ hour });
      val = this.getProg(dow, this.getProgIdx(time));
      this.data.push( [ time.format('H:mm'), val+1 ] ); // val 1..3

      time = moment({ hour, minute: 30 });
      val = this.getProg(dow, this.getProgIdx(time));
      this.data.push( [ time.format('H:mm'), val+1 ] ); // val 1..3

    }
    console.log(`${dow} ${this.data}`);
    if (forceRefresh){
      this.data = Object.assign([], this.data);
    }
  }

  room: any;
  deviceId: any;
  roomId: any;
  dataTimerId: any;
  weatherTimerId: any;
  weather : any;

  @ViewChild('chart', {static: false}) chart!: GoogleChartComponent; // non-null assertion entity

  constructor(private dataService: DataService, private route: ActivatedRoute, private router: Router) { }

  ngOnInit(): void {
    // For now refreshData with a timer, in future we probably should use an Observable..
    this.refreshData();
    this.dataTimerId = setInterval(() => { this.refreshData(); }, 5000);

    this.refreshWeatherData();
    this.weatherTimerId = setInterval(() => { this.refreshWeatherData(); }, 600000);
  }

  ngOnDestroy(): void {
    clearInterval(this.dataTimerId);
    clearInterval(this.weatherTimerId);
  }

  refreshData(){
    this.dataService.getRoom(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId).subscribe((data: any) =>{
      console.log(data);
      this.room = data;
      this.deviceId = this.route.snapshot.params.deviceId;
      this.roomId = this.route.snapshot.params.roomId;

      this.updateGraph();
    })
  }

  refreshWeatherData(){
    this.dataService.getWeather().subscribe((data: any) => {
      console.log(data);
      this.weather = data
    })
  }

  displayTemp(temp: any){
    temp = temp/10.0;
    if (this.room.units){
      temp = temp*9.0/5.0 + 32.0;
    }
    return temp.toFixed(1); 
  }

  t1Up(){
    this.room.t1 = this.room.t1+2;
    this.dataService.setT1(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, this.room.t1);
  }
  t1Down(){
    this.room.t1 = this.room.t1-2;
    this.dataService.setT1(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, this.room.t1);
  }

  t2Up(){
    this.room.t2 = this.room.t2+2;
    this.dataService.setT2(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, this.room.t2);
  }
  t2Down(){
    this.room.t2 = this.room.t2-2;
    this.dataService.setT2(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, this.room.t2);
  }

  t3Up(){
    this.room.t3 = this.room.t3+2;
    this.dataService.setT3(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, this.room.t3);
  }
  t3Down(){
    this.room.t3 = this.room.t3-2;
    this.dataService.setT3(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, this.room.t3);
  }

  setUnits(val: any){
    this.room.units = val;
    this.dataService.setUnits(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, this.room.units);
  }
  setBoost(val: any){
    //This will always fail with a 405 because there is no underlying api to control this remotely
    this.room.boost = val; 
    this.dataService.setBoost(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, this.room.boost);
  }
  setAdvance(val: any){
    this.room.advance = val;
    this.dataService.setAdvance(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, this.room.advance);
  }

  setSeason(val: any){
    this.room.winter = val;
    this.dataService.setSeason(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, this.room.winter);
  }
  setMode(val: any){
    this.room.mode = val;
    this.dataService.setMode(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, this.room.mode);
  }
  setProg(dow: number, val: any){
    this.dataService.setProg(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, dow, val);
  }
}
