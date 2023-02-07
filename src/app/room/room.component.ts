import { Component, OnInit } from '@angular/core';
import { DataService } from '../data.service';
import { ActivatedRoute, Router } from '@angular/router';

@Component({
  selector: 'app-room',
  templateUrl: './room.component.html',
  styleUrls: ['./room.component.css']
})
export class RoomComponent implements OnInit {

  deviceId: any;
  rooms: any[] = [];

  constructor(private dataService: DataService, private route: ActivatedRoute, private router: Router) { }

  ngOnInit(): void {
    this.dataService.getRooms(this.route.snapshot.params.deviceId).subscribe((data: any) =>{
      console.log(data);
      this.rooms = data;
      this.deviceId = this.route.snapshot.params.deviceId;
    })
  }

}

