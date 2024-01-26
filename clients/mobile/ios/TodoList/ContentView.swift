//
//  ContentView.swift
//  TodoList
//
//  Created by Alexandre Hanot on 18/01/2024.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text(sayHi())
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
