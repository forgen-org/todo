//
//  ContentView.swift
//  TodoList
//
//  Created by Alexandre Hanot on 18/01/2024.
//

import SwiftUI

struct ContentView: View {
  @State private var errorMessage: String = ""
  @State private var newTaskName: String = ""
  @State private var showError: Bool = false
  @State private var todolist: TodoListDto?

  var client = Client()

  var body: some View {
    NavigationView {
      VStack {
        List {
          ForEach(todolist?.tasks ?? [], id: \.index) { task in
            HStack {
              Text(task.name)

              Spacer()

              Toggle(
                "",
                isOn: Binding(
                  get: { task.status == .completed },
                  set: { _, _ in
                    Task {
                      do {
                        try await client.completeTask(index: task.index)
                        todolist = try await client.getTodoList()
                      } catch {
                        handleError(error)
                      }
                    }
                  })
              )
              .labelsHidden()
            }
          }
        }

        HStack {
          TextField("Enter task name", text: $newTaskName)
            .textFieldStyle(RoundedBorderTextFieldStyle())
            .padding()

          Button(
            action: {
              Task {
                do {
                  try await client.addTask(name: newTaskName)
                  todolist = try await client.getTodoList()
                  newTaskName = ""
                } catch {
                  handleError(error)
                }
              }
            },
            label: {
              Text("Add")
            }
          )
          .padding()
        }
      }
      .navigationBarTitle("TodoList")
      .alert(isPresented: $showError) {
        Alert(
          title: Text("Error"), message: Text(errorMessage), dismissButton: .default(Text("OK")))
      }
    }
  }

  private func handleError(_ error: Error) {
    if let errorDto = error as? ErrorDto {
      switch errorDto {
      case .Error(let message):
        errorMessage = message
      }
      showError = true
    } else {
      // Handle other errors
      errorMessage = error.localizedDescription
      showError = true
    }
  }

}

#Preview {
  ContentView()
}
