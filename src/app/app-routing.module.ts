import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AboutComponent } from './about/about.component';
import { DeviceComponent } from './device/device.component';
import { DeviceDetailsComponent } from './device-details/device-details.component';
import { RoomComponent } from './room/room.component';
import { RoomDetailsComponent } from './room-details/room-details.component';


const routes: Routes = [
  { path: '', redirectTo: 'devices', pathMatch: 'full' },
  { path: 'about', component: AboutComponent },
  { path: 'devices', component: DeviceComponent, data: { title: 'Device List' } },
  { path: 'devices/:id', component: DeviceDetailsComponent, data: { title: 'Device Details' } },
  { path: 'devices/:deviceId/rooms', component: RoomComponent, data: { title: 'Room List' } },
  { path: 'devices/:deviceId/rooms/:roomId', component: RoomDetailsComponent, data: { title: 'Thermostat Status' } },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
