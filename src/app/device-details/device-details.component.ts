import { Component, OnInit } from '@angular/core';
import { DataService } from '../data.service';
import { ActivatedRoute, Router } from '@angular/router';

@Component({
  selector: 'app-device-details',
  templateUrl: './device-details.component.html',
  styleUrls: ['./device-details.component.css']
})
export class DeviceDetailsComponent implements OnInit {

  device: any;
  id: any;
  lastseen: Date = new Date(0);
  dataTimerId: any;

  constructor(private dataService: DataService, private route: ActivatedRoute, private router: Router) { }

  ngOnInit(): void {
    this.refreshData();
    this.dataTimerId = setInterval(() => { this.refreshData(); }, 5000);
  }
    
  ngOnDestroy(): void {
    clearInterval(this.dataTimerId);
  }
    
  refreshData(){
    this.dataService.getDevice(this.route.snapshot.params.id).subscribe((data: any) =>{
      console.log(data);
      this.device = data;
      this.id = this.route.snapshot.params.id;
      this.lastseen = new Date(data.lastseen*1000);
    })
  } 




}
