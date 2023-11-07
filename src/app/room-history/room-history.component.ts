import { Component, OnInit, ViewChild } from '@angular/core';
import { DataService } from '../data.service';
import { ActivatedRoute, Router } from '@angular/router';
import { GoogleChartComponent, ChartType, ChartSelectionChangedEvent } from 'angular-google-charts';
import { MatDatepickerInputEvent } from '@angular/material/datepicker';
import * as moment from 'moment';

export interface DataPoint {
  ts: Date;
  temp: number;
  settemp: number; 
  heating: number;
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
  columnNames = ['Time', 'Temp', 'SetTemp', 'OutsideTemp', 'HeatingRequest' ];
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
    'explorer' : {   // NB Does not work with touch/gestures :(
      'axis' : 'horizontal',
      'keepInBounds' : true,
      'actions' : [ 'dragToZoom', 'rightClickToReset' ],
    },
  };  

  onChartSelect(event: ChartSelectionChangedEvent) {
  }


  updateGraph(roomHistory : any, weatherHistory : any) : void {
    // [{"ts": "2023-11-02T12:54:36.182454+00:00", "temp": 186, "settemp": 190}, 
    this.data = new Array<Array<Date | number | null>>();

    roomHistory.forEach( (elem: DataPoint) => {
      // hack: if a series is all null the chart will not render
      var heating: number;
      if (elem.heating) { heating = elem.heating; } else { heating = 0; }  
      let v = new Array<Date|number|null>(new Date(elem.ts), elem.temp, elem.settemp, null, heating);
      this.data.push(v);
    });
    weatherHistory.forEach( (elem: WeatherDataPoint) => {
      let v = new Array<Date|number|null>(new Date(elem.ts), null, null, elem.temp, null);
      this.data.push(v);
    });

    this.data = Object.assign([], this.data); // force refresh
  }

  deviceId: any;
  roomId: any;
  historyTimerId: any;
  startDate: Date = moment().subtract(2,'days').toDate();
  endDate: Date | null = null;

  @ViewChild('chart', {static: false}) chart!: GoogleChartComponent; // non-null assertion entity

  constructor(private dataService: DataService, private route: ActivatedRoute, private router: Router) { }

  ngOnInit(): void {
    this.refreshHistoryData();
    this.historyTimerId = setInterval(() => { this.refreshHistoryData(); }, 60000);
  }

  ngOnDestroy(): void {
    clearInterval(this.historyTimerId)
  }

  refreshHistoryData(){
    var from_date : string = this.startDate.toISOString();
    var to_date : string | null = null;
    if (this.endDate) {
      to_date = this.endDate.toISOString();
    }

    // @todo Use concatMap, switchMap etc. so we do not nest the observables...
    this.dataService.getRoomHistory(this.route.snapshot.params.deviceId, this.route.snapshot.params.roomId, from_date, to_date).subscribe((roomHistory: any) =>{
      console.log(roomHistory);
      this.deviceId = this.route.snapshot.params.deviceId;
      this.roomId = this.route.snapshot.params.roomId;

      this.dataService.getWeatherHistory(from_date,to_date).subscribe((weatherHistory: any) =>{
        console.log(weatherHistory);
        this.updateGraph(roomHistory,weatherHistory);
      });
    });

  }

  updateStartDate(event: MatDatepickerInputEvent<Date>) {
    if (event.value) {
      this.startDate = event.value;
      this.refreshHistoryData();
    }
  }

  updateEndDate(event: MatDatepickerInputEvent<Date>) {
    if (event.value) {
      this.endDate = event.value;
      this.refreshHistoryData();
    }
  }
}
