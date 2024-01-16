//
//  ContentView.swift
//  TodoList
//
//  Created by Alexandre Hanot on 17/01/2024.
//

import SwiftUI

struct ContentView: View {
    @EnvironmentObject var rustApp: RustAppWrapper
    
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text(rustApp.rust.say_hello().toString())
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
