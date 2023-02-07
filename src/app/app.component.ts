import { Component } from '@angular/core';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'besmart-thermostat';

  time = new Date();
  interval: any;

  getDay() : string {
    const day = [ "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday" ];
    return day[this.time.getDay()];
  }

  ngOnInit() {
    this.time = new Date();
    this.interval = setInterval(() => { this.time = new Date(); }, 5000);
  }
}
