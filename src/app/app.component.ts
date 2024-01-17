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
  greetingMessage = "";
  openAddNew: boolean=false;
  task:string='';
  constructor(){
    this.connect_db();
  }
  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;
    });
  }
  connect_db(){
    // invoke<string>("create_todo",{todo:"first todo task"}).then((text) => {
    //   this.greetingMessage = text;
    // }); 
    invoke<string>("get_todo").then((text) => {
      this.greetingMessage = text;
    });
  }
  addNewTask(){
    this.openAddNew=true;
    const today = new Date()
    let todo={
      task:"adding task from frontend",
      added_at:today.valueOf().toString()
    }
     invoke<string>("create_todo",{todo}).then((text) => {
     console.log(text)
    });
    console.log('called')
  }
  saveTask(){
    
  }
}
