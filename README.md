# Warfarin App

[![Powered by Rust + WASM](https://img.shields.io/badge/powered%20by-Rust%20%2B%20WASM-orange?style=for-the-badge&logo=rust)](https://webassembly.org)
[![Built with Vue.js](https://img.shields.io/badge/built%20with-Vue.js-green?style=for-the-badge&logo=vue.js)](https://vuejs.org/)
[![Deploy with Vercel](https://img.shields.io/badge/deploy%20with-Vercel-black?style=for-the-badge&logo=vercel)](https://vercel.com)

A modern, high-performance web application for calculating complex Warfarin dosage regimens. This project uniquely combines a **Vue.js** frontend for a reactive user experience with a powerful **Rust-powered WebAssembly (WASM)** core for all logical computations, ensuring maximum performance, safety, and reliability.

The application is designed to assist healthcare professionals by generating multiple, prioritized dosing options based on various parameters like target weekly dose, available pill strengths, and special dosing patterns.

## ✨ Features

- **High-Performance Core**: All complex calculations are handled by a Rust-powered WebAssembly module, offering near-native speed directly in the browser.
- **Advanced Dosing Algorithms**: Generates both uniform and non-uniform (e.g., special dose days, stop days) weekly regimens.
- **Smart Option Prioritization**: Results are intelligently sorted based on complexity (e.g., minimizing half-pills, using fewer pill strengths).
- **Automatic Dose Adjustment**: Quickly calculate new weekly doses based on percentage changes from the previous dose.
- **Appointment-Based Pill Counting**: Calculates the exact number of pills required until the next appointment date.
- **Flexible Configuration**: Users can select available pill strengths (1mg, 2mg, 3mg, 5mg) and toggle the use of half-pills.
- **Interactive UI**: A fully reactive interface built with Vue.js for a seamless user experience.
- **Zero Backend Dependency**: The entire application runs on the client-side, making it fast, private, and easy to deploy on static hosts.

## ⚠️ Medical Disclaimer

**This application is intended for educational and demonstration purposes only.** It must **NOT** be used for actual medical decision-making. Warfarin dosage adjustments must always be performed by a qualified healthcare professional. The developers assume no liability for any misuse of this tool.

## 🛠️ Tech Stack

- **Frontend**: [Vue.js 3](https://vuejs.org/) (with Composition API and Scoped CSS)
- **Build Tool**: [Vite](https://vitejs.dev/)
- **Logic Core**: [Rust](https://www.rust-lang.org/) compiled to [WebAssembly](https://webassembly.org/)
- **Deployment**: [Vercel](https://vercel.com/)

The core principle of this project is to separate concerns: Vue.js manages the user interface and state, while Rust handles the heavy, mission-critical calculations, providing a robust and safe foundation.

## 🚀 Getting Started

Follow these instructions to get a local copy up and running for development purposes.

### Prerequisites

You need the following tools installed on your system:

1.  **Node.js**: [Download & Install Node.js](https://nodejs.org/) (LTS version recommended).
2.  **Rust**: Install Rust and Cargo via `rustup`.
    ```bash
    # For Linux/macOS/WSL
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    # For Windows, download from https://rustup.rs/
    ```
3.  **wasm-pack**: A tool for building Rust-generated WebAssembly.
    ```bash
    cargo install wasm-pack
    ```

### Installation & Local Development

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/pharmacist-sabot/warfarin-app.git
    cd warfarin-app
    ```

2.  **Install frontend dependencies:**

    ```bash
    bun install
    ```

3.  **Build the WebAssembly module:**
    This command compiles the Rust code in the `warfarin_logic` directory into a WASM package inside `warfarin_logic/pkg`.

    ```bash
    wasm-pack build ./warfarin_logic --target web
    ```

    _Note: You only need to re-run this command after making changes to the Rust (`.rs`) files._

4.  **Run the development server:**
    This will start a Vite dev server, typically on `http://localhost:5173`.
    ```bash
    bun run dev
    ```

Open your browser and navigate to the provided local URL to see the app in action!

## 🏗️ Project Structure

```
.
├── warfarin_logic/       # Rust crate for all calculation logic
│   ├── src/
│   │   └── lib.rs        # The heart of the application logic
│   └── Cargo.toml      # Rust dependencies
│
├── src/                  # Vue.js application source
│   ├── components/
│   └── App.vue           # Main Vue component
│
├── public/               # Static assets
├── vite.config.js        # Vite build configuration
├── package.json          # Node.js project manifest
└── README.md             # This file
```

## 🌐 Deployment

This project is configured for seamless deployment on [Vercel](https://vercel.com/).

1.  **Push your code to a GitHub repository.**
2.  **Import the repository into Vercel.**
3.  **Configure the build settings:**
    - **Framework Preset**: `Vite`
    - **Build Command**: `bun run build`
    - **Output Directory**: `dist`
    - **Install Command**: `bun install`

    **Crucially, the install command must build the WASM module before Vite builds the frontend.**

4.  **Deploy!** Vercel will handle the rest.

## 🤝 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

## 📄 License

This project is distributed under the MIT License.
