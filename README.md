# api-dev-tool
API testing and development tool

# Project Structure
src/
├── main.rs            # Application entry point, set up initial state and call the UI library
├── lib.rs             # Main application logic, contains business logic without the UI component
├── client/
│   ├── mod.rs         # ProtocolClient trait + re-exports
│   ├── http.rs        # HttpClient impl
│   ├── graphql.rs     # GraphQLClient impl
│   └── grpc.rs        # GrpcClient impl
├── app/
│   ├── mod.rs         # Module declarations
│   ├── state.rs       # AppState struct
│   └── message.rs     # Message enum
├── ui/
│   ├── mod.rs         # Module declarations
│   ├── view.rs        # Main view function
│   └── screens.rs     # Screen components
├── storage/
│   ├── mod.rs         # Module declarations
│   └── file.rs        # FileStorage implementation
├── error.rs           # Application's error definitions
└── config.rs          # Configuration management