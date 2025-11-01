# VaultMind

VaultMind is a secure, local-first desktop application for managing sensitive data, including passwords, secure notes, and encrypted files. It is built with a focus on privacy, ensuring that all user data is encrypted and stored exclusively on the user's machine.

## Core Features

-   **Local-First Storage**: All data is stored in a local SQLite database, ensuring privacy and offline access.
-   **Strong Encryption**: Implements AES-256-GCM for data encryption and Argon2id for key derivation.
-   **Cross-Platform**: Built with Tauri, allowing it to run on Windows, macOS, and Linux.
-   **Modular Design**: Features separate modules for different data types, including a password vault, secure notes, and encrypted file storage.
-   **Modern UI**: A clean and intuitive user interface built with React and TypeScript, styled with TailwindCSS.

## Technology Stack

-   **Frontend**: React, TypeScript, TailwindCSS, Shadcn/UI
-   **Backend (Core)**: Rust, Tauri
-   **Database**: SQLite / SQLCipher
-   **Build Tool**: Vite

## Project Status

The project is currently in the initial development phase. The foundational setup is complete, including:

-   Tauri project initialization with a React/TypeScript template.
-   Integration of TailwindCSS for styling.
-   Basic component and directory structure.

The next steps involve implementing the core application logic, database integration, and encryption functionalities.

## Getting Started

To get a local copy up and running, follow these steps.

### Prerequisites

-   [Node.js](https://nodejs.org/)
-   [Rust](https://www.rust-lang.org/tools/install)
-   [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation & Execution

1.  Clone the repository:
    ```sh
    git clone <repository-url>
    ```
2.  Navigate to the project directory:
    ```sh
    cd VaultMind
    ```
    
3.  Install NPM packages:
    ```sh
    npm install
    ```

4.  Run the development server:
    ```sh
    npm run tauri dev
    ```