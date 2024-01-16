import SwiftUI

@main
struct TodoListApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(RustAppWrapper(rust: RustApp()))
        }
    }
}

class RustAppWrapper: ObservableObject {
    var rust: RustApp

    init (rust: RustApp) {
        self.rust = rust
    }
}
