//
//  ContentView.swift
//  TodoList
//
//  Created by Alexandre Hanot on 18/01/2024.
//

import SwiftUI

struct ContentView: View {
  @State private var todolist: String = "Loading..."

  var client = Client()

  var body: some View {
      NavigationView {
                  VStack {
                      List(tasks) { task in
                          Text(task.name)
                      }

                      HStack {
                          TextField("Enter task name", text: $newTaskName)
                              .textFieldStyle(RoundedBorderTextFieldStyle())
                              .padding()

                          Button(action: addTask) {
                              Text("Send")
                          }
                          .padding()
                      }
                  }
                  .navigationBarTitle("TodoList")
              }
      
    VStack {
      Text(todolist)
      Button("Add Task") {
        Task {
            await addTask()
        }
      }

    }
    .padding()
    .onAppear {
      loadTodoList()
    }
  }

  private func loadTodoList() {
    Task {
      todolist = await client.getTodolist()
    }
  }

  private func addTask() async {
    await client.addTask(name: "Hello iOS")
    let res = await client.getTodolist()
    todolist = res
}

}

#Preview {
  ContentView()
}
