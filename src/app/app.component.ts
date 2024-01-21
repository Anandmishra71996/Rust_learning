import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from "@tauri-apps/api/tauri";
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet,FormsModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  taskList:any[] = [];
  openAddNew: boolean=false;
  task:string='';
  constructor(){
    this.getTasks();
  }

  getTasks(){
    // invoke<string>("create_todo",{todo:"first todo task"}).then((text) => {
    //   this.greetingMessage = text;
    // }); 
    invoke<string>("get_todo").then((res:any) => {
      this.taskList = res;
      console.log(this.taskList)
    });
  }
  addNewTask(){
    this.openAddNew=true;
   
    console.log('called')
  }
  saveTask(){
    const today = new Date()
    let todo={
      task:this.task,
      added_at:today.valueOf().toString()
    }
     invoke<string>("create_todo",{todo}).then((text) => {
     console.log(text)
     this.getTasks();
    });
  }
}
