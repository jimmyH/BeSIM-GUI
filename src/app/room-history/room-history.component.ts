import { Component, OnInit, ViewChild } from '@angular/core';
import { DataService } from '../data.service';
import { ActivatedRoute, Router } from '@angular/router';
import { GoogleChartComponent, ChartType, ChartSelectionChangedEvent } from 'angular-google-charts';
import { MatSelectChange } from '@angular/material/select';
import * as moment from 'moment';

export interface DataPoint {
  ts: Date;
  temp: number;
  settemp: number; 
}

export interface WeatherDataPoint {
  ts: Date;
  temp: number;
}

@Component({
  selector: 'app-room-history',
  templateUrl: './room-history.component.html',
  styleUrls: ['./room-history.component.css']
})

export class RoomHistoryComponent implements OnInit {

  type: ChartType = ChartType.LineChart
  data = new Array<Array<Date | number | null>>();
  columnNames = ['Time', 'Temp', 'SetTemp', 'OutsideTemp' ];  
  options = {      
    'vAxis' : { 'title' : 'Temperature',
                'minValue' : 10, 'maxValue' : 20,
                //'viewWindow' : { 'max' : 30, 'min' : 10 }
               },
    'hAxis' : { 'title' : 'Time' },
    //'legend' : { 'position': 'top' },
    //'vAxis' :  { 'textPosition': 'left',
    //             'gridlines' : { 'color': 'transparent' },
    //             'ticks' : [ 0, 1, 2, 3 ]
    //           },
    //'hAxis' :  { 'textPosition': 'left',
    //          },
    //'backgroundColor' : 'transparent',
    //'bar' : { 'groupWidth' : '90%' },
    //'chartArea' : { 'width' : '80%', 'height' : '80%' },
    'width' : '100%',
    'height' : '100%',
  };  

  onChartSelect(event: ChartSelectionChangedEvent) {
  }


  updateGraph(forceRefresh: boolean = false): void{
    // [{"ts": "2023-11-02T12:54:36.182454+00:00", "temp": 186, "settemp": 190}, 
    //while (this.data.length>0) { this.data.pop(); }

    this.data = new Array<Array<Date | number | null>>();

    this.roomHistory.forEach( (elem: DataPoint) => {
      let v = new Array<Date|number|null>(new Date(elem.ts), elem.temp, elem.settemp, null);
      this.data.push(v);
    });
    this.weatherHistory.forEach( (elem: WeatherDataPoint) => {
      let v = new Array<Date|number|null>(new Date(elem.ts), null, null, elem.temp);
      this.data.push(v);
    });

    if (forceRefresh){
      this.data = Object.assign([], this.data);
    }
  }

  roomHistory: any;
  deviceId: any;
  roomId: any;
  roomHistoryTimerId: any;
  weatherHistory: any;
  weatherHistoryTimerId: any;

  @ViewChild('chart', {static: false}) chart!: GoogleChartComponent; // non-null assertion entity

  constructor(private dataService: DataService, private route: ActivatedRoute, private router: Router) { }

  ngOnInit(): void {
    this.refreshRoomHistoryData();
    this.roomHistoryTimerId = setInterval(() => { this.refreshRoomHistoryData(); }, 60000);
    this.refreshWeatherHistoryData();
    this.weatherHistoryTimerId = setInterval(() => { this.refreshWeatherHistoryData(); }, 600000);
  }

  ngOnDestroy(): void {
    clearInterval(this.roomHistoryTimerId)
    clearInterval(this.weatherHistoryTimerId)
  }

  refreshRoomHistoryData(){
    var date : string = moment().subtract(2,'weeks').format(); // @todo make configurable
    this.dataService.getRoomHistory(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, date).subscribe((data: any) =>{
      console.log(data);
      this.roomHistory = data;
      this.deviceId = this.route.snapshot.params.deviceId;
      this.roomId = this.route.snapshot.params.roomId;
      this.updateGraph();
    })
  }

  refreshWeatherHistoryData(){
    var date : string = moment().subtract(2,'weeks').format(); // @todo make configurable
    this.dataService.getWeatherHistory(date).subscribe((data: any) =>{
      console.log(data);
      this.weatherHistory = data;
      this.updateGraph();
    })
  }
}
